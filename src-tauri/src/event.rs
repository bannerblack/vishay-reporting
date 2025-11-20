use sea_orm::{Set, ActiveModelTrait, EntityTrait, QueryFilter, ColumnTrait};
use entity::event;
use serde::{Deserialize, Serialize};
use tauri::State;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct EventData {
    pub originator_id: Option<i32>,
    pub target_id: Option<i32>,
    pub report_id: i32,
    pub comment: String,
}

#[derive(Debug, Serialize)]
pub struct EventResponse {
    pub id: i32,
    pub originator_id: Option<i32>,
    pub target_id: Option<i32>,
    pub report_id: i32,
    pub comment: String,
    pub created_at: String,
    pub complete: bool,
    pub completed_date: Option<String>,
}

#[tauri::command]
pub async fn create_event(state: State<'_, AppState>, event_data: EventData) -> Result<EventResponse, String> {
    let db = &*state.core_db;

    let event = event::ActiveModel {
        originator_id: Set(event_data.originator_id),
        target_id: Set(event_data.target_id),
        report_id: Set(event_data.report_id),
        comment: Set(event_data.comment),
        complete: Set(false),
        completed_date: Set(None),
        ..Default::default()
    };

    let event: event::Model = event
        .insert(db)
        .await
        .map_err(|e| format!("Failed to create event: {}", e))?;

    Ok(EventResponse {
        id: event.id,
        originator_id: event.originator_id,
        target_id: event.target_id,
        report_id: event.report_id,
        comment: event.comment,
        created_at: event.created_at.to_string(),
        complete: event.complete,
        completed_date: event.completed_date.map(|d| d.to_string()),
    })
}

#[tauri::command]
pub async fn get_event(state: State<'_, AppState>, id: i32) -> Result<EventResponse, String> {
    let db = &*state.core_db;

    let event = event::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch event: {}", e))?
        .ok_or_else(|| "Event not found".to_string())?;

    Ok(EventResponse {
        id: event.id,
        originator_id: event.originator_id,
        target_id: event.target_id,
        report_id: event.report_id,
        comment: event.comment,
        created_at: event.created_at.to_string(),
        complete: event.complete,
        completed_date: event.completed_date.map(|d| d.to_string()),
    })
}

#[tauri::command]
pub async fn get_all_events(state: State<'_, AppState>) -> Result<Vec<EventResponse>, String> {
    let db = &*state.core_db;

    let events = event::Entity::find()
        .all(db)
        .await
        .map_err(|e| format!("Failed to fetch events: {}", e))?;

    Ok(events
        .into_iter()
        .map(|event| EventResponse {
            id: event.id,
            originator_id: event.originator_id,
            target_id: event.target_id,
            report_id: event.report_id,
            comment: event.comment,
            created_at: event.created_at.to_string(),
            complete: event.complete,
            completed_date: event.completed_date.map(|d| d.to_string()),
        })
        .collect())
}

#[derive(Debug, Deserialize)]
pub struct UpdateEventData {
    pub comment: Option<String>,
    pub complete: Option<bool>,
}

#[tauri::command]
pub async fn update_event(state: State<'_, AppState>, id: i32, event_data: UpdateEventData) -> Result<EventResponse, String> {
    let db = &*state.core_db;

    let event = event::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch event: {}", e))?
        .ok_or_else(|| "Event not found".to_string())?;

    let mut event: event::ActiveModel = event.into();
    
    if let Some(comment) = event_data.comment {
        event.comment = Set(comment);
    }
    
    if let Some(complete) = event_data.complete {
        event.complete = Set(complete);
        if complete {
            event.completed_date = Set(Some(chrono::Utc::now().naive_utc()));
        } else {
            event.completed_date = Set(None);
        }
    }

    let event: event::Model = event
        .update(db)
        .await
        .map_err(|e| format!("Failed to update event: {}", e))?;

    Ok(EventResponse {
        id: event.id,
        originator_id: event.originator_id,
        target_id: event.target_id,
        report_id: event.report_id,
        comment: event.comment,
        created_at: event.created_at.to_string(),
        complete: event.complete,
        completed_date: event.completed_date.map(|d| d.to_string()),
    })
}

#[tauri::command]
pub async fn complete_event(state: State<'_, AppState>, id: i32) -> Result<EventResponse, String> {
    let db = &*state.core_db;

    let event = event::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch event: {}", e))?
        .ok_or_else(|| "Event not found".to_string())?;

    let mut event: event::ActiveModel = event.into();
    event.complete = Set(true);
    event.completed_date = Set(Some(chrono::Utc::now().naive_utc()));

    let event: event::Model = event
        .update(db)
        .await
        .map_err(|e| format!("Failed to complete event: {}", e))?;

    Ok(EventResponse {
        id: event.id,
        originator_id: event.originator_id,
        target_id: event.target_id,
        report_id: event.report_id,
        comment: event.comment,
        created_at: event.created_at.to_string(),
        complete: event.complete,
        completed_date: event.completed_date.map(|d| d.to_string()),
    })
}

#[tauri::command]
pub async fn delete_event(state: State<'_, AppState>, id: i32) -> Result<String, String> {
    let db = &*state.core_db;

    let event = event::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| format!("Failed to fetch event: {}", e))?
        .ok_or_else(|| "Event not found".to_string())?;

    let event: event::ActiveModel = event.into();
    event
        .delete(db)
        .await
        .map_err(|e| format!("Failed to delete event: {}", e))?;

    Ok(format!("Event {} deleted successfully", id))
}

#[tauri::command]
pub async fn get_events_by_user(state: State<'_, AppState>, user_id: i32) -> Result<Vec<EventResponse>, String> {
    let db = &*state.core_db;

    let events = event::Entity::find()
        .filter(event::Column::TargetId.eq(user_id))
        .all(db)
        .await
        .map_err(|e| format!("Failed to fetch events for user: {}", e))?;

    Ok(events
        .into_iter()
        .map(|event| EventResponse {
            id: event.id,
            originator_id: event.originator_id,
            target_id: event.target_id,
            report_id: event.report_id,
            comment: event.comment,
            created_at: event.created_at.to_string(),
            complete: event.complete,
            completed_date: event.completed_date.map(|d| d.to_string()),
        })
        .collect())
}