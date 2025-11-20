import type { PageLoad } from './$types';
import { authenticateUser, needsInitialSetup } from '$lib/db/adapters/auth';
import { newEventSchema } from '$types';
import { superValidate } from 'sveltekit-superforms';
import { zod4 } from 'sveltekit-superforms/adapters';

export const ssr = false;

export const load: PageLoad = async () => {
    try {
        const authenticatedUser = await authenticateUser();

        const form = await superValidate(
            {
                originator_id: authenticatedUser.id, 
                target_id: null, 
                report_id: 0, 
                comment: ''
            }, 
            zod4(newEventSchema)
        );

        return { 
            user: authenticatedUser,
            form
        };
    } catch (err) {
        console.error('Error loading page data:', err);
        
        const needsSetup = await needsInitialSetup();
        if (needsSetup) {
            const form = await superValidate({}, zod4(newEventSchema));
            return {
                user: null,
                isInitialSetup: true,
                form
            };
        }
        
        const form = await superValidate({}, zod4(newEventSchema));
        return { 
            user: null,
            form,
            error: 'User not registered in system'
        };
    }
};