import { error } from '@sveltejs/kit';
import { superValidate } from 'sveltekit-superforms';
import { zod4 } from 'sveltekit-superforms/adapters';
import { invoke } from '@tauri-apps/api/core';
// import { z } from 'zod';

import type { winUser } from '../types/types';
import { _userSchema } from '../types/types';

export const load = async ({ fetch }) => {
	//   const id = parseInt(params.id);
	const user: winUser = await invoke('get_system_user');
	const permissions: string[] = await invoke('get_user_roles');

	const request = await fetch(`https://jsonplaceholder.typicode.com/users/1`);
	if (request.status >= 400) throw error(request.status);

	const userData = await request.json();
	const form = await superValidate(userData, zod4(_userSchema));

	return { user, form, permissions };
};
