// import { error } from '@sveltejs/kit';
// import { superValidate } from 'sveltekit-superforms';
// import { zod4 } from 'sveltekit-superforms/adapters';
// import { invoke } from '@tauri-apps/api/core';
import { z } from 'zod';

export type eTest = {
	type: string;
	frequency: number;
	voltage: number;
	min: number;
	max: number;
	output_uom: string;
	prim_pins: string;
	sec_pins: string;
	shorted_pins: string;
	description: string;
};

export type fgNew = {
	fg: string;
	rev: string;
	customer: string;
	attributes: string[];
	tests: eTest[];
};

export const eTestSchema = z.object({
	type: z.string(),
	frequency: z.number(),
	voltage: z.number(),
	min: z.number(),
	max: z.number(),
	output_uom: z.string(),
	prim_pins: z.string(),
	sec_pins: z.string(),
	shorted_pins: z.string(),
	description: z.string()
});

export const newFgSchema = z.object({
	fg: z.string().min(1),
	rev: z.string().min(1),
	customer: z.string(),
	attributes: z.array(z.string()),
	tests: z.array(eTestSchema)
});

export const _userSchema = z.object({
	id: z.number().int().positive(),
	name: z.string().min(2),
	email: z.string().email()
});

export type winUser = [string, string];

// get type from _userSchema
export type User = z.infer<typeof _userSchema>;
