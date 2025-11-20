// Parse Manual Data
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManualTest {
    pub result: i32,
    pub test: String,
    pub fg: String,
    pub rev: String,
    pub batch: String,
    pub operator: String,
    pub date: String,
    pub time: String,
    pub sn: String,
    pub passfail: String,
    pub minimum: f64,
    pub reading: f64,
    pub maximum: f64,
    pub uom: String,
}

pub fn get_manual_tests_fg(fg : String) {
    // Labview path is \\wsdv03\DV_Specific\Departments\Voltech\LabView\LabView Results\{FG}\filenames.csv
    // Example file: "LabView Results\138090\25-11-19 138090-LFT-LL.csv"
    // THe format is YY-MM-DD FG-LFT-{TEST TYPE}-{Optional Number if multiple of the same}
    // Check if dir exists
    // Parse all the files in the dir
    // Raw Data Example: 6,138090-LFT-LL,138090,A,162527,PJD,11/19/2025,7:51:33 AM,3398,PASS,384,486.850000,576,nH
    // There may or may not be a header, which would start with "#"
    // Return test rows that are filtred by batch, test name, sn, date, etc.
}

