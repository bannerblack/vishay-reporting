import type { PageLoad } from './$types';
import { getReportWithTests } from '$lib/db/database';
import { superValidate } from 'sveltekit-superforms';
import { zod4 } from 'sveltekit-superforms/adapters';
import { newReportSchema } from '$types';

export const load = (async ({ params }) => {
    const [report] = await Promise.all([
        getReportWithTests(Number(params.id))
    ]);

    const form = await superValidate(
        {
            originator_id: 1,
            report_id: Number(params.id),
            sn_range: ''
        },
        zod4(newReportSchema)
    );

    return { report, form };
}) satisfies PageLoad;

