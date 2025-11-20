// Internationalization strings
// Currently supports: English (en)
// To add a new language, create a new object with the same structure

export type Language = 'en';

export const strings = {
    en: {
        common: {
            loading: 'Loading...',
            search: 'Search',
            searching: 'Searching...',
            back: 'Back',
            submit: 'Submit',
            cancel: 'Cancel',
            save: 'Save',
            delete: 'Delete',
            edit: 'Edit',
            view: 'View',
            all: 'All',
            actions: 'Actions',
            viewDetails: 'View Details',
            viewAll: 'View All',
            noResults: 'No results found',
            error: 'Error',
            success: 'Success'
        },

        // Batch-related strings
        batches: {
            title: 'Batches',
            subtitle: 'Browse and search batch test data',
            batchNumber: 'Batch Number',
            partNumber: 'Part Number',
            date: 'Date',
            operator: 'Operator',
            totalTests: 'Total Tests',
            passRate: 'Pass Rate',
            passed: 'passed',
            failed: 'failed',
            searchPlaceholder: 'Search batches...',
            noMatches: 'No batches match your search.',
            notFound: 'No batches found.',
            viewDetails: 'View Details',
            allTests: 'All',
            backToBatches: 'Back to Batches',
            detailedResults: 'Detailed test results for this batch',
            resultsForDate: 'Test results for',
            allResultsForBatch: 'All test results for this batch',
            viewAllTests: 'View All Tests',
            allTestsTitle: 'All Tests',
            allTestsSubtitle: 'All test results across all dates for this batch',
            testDates: 'Test Dates',
            partsTested: 'Parts Tested',
            operators: 'Operators',
            overallPassRate: 'Overall Pass Rate',
            testDatesBreakdown: 'Test Dates',
            testedOnDates: 'This batch was tested on',
            differentDate: 'different date',
            differentDates: 'different dates',
            allTestResults: 'All Test Results',
            acrossAllDates: 'Across All Dates',
            totalBatches: 'Total Batches'
        },

        // Parts-related strings
        parts: {
            title: 'Parts',
            subtitle: 'Browse and search part test data',
            totalParts: 'Total Parts',
            searchPlaceholder: 'Search parts...'
        },

        // Test results
        tests: {
            serialNumber: 'Serial Number',
            testName: 'Test Name',
            result: 'Result',
            resultNum: 'Result #',
            timestamp: 'Timestamp',
            file: 'File',
            pass: 'Pass',
            fail: 'Fail',
            noResults: 'No test results found for this batch.'
        },

        // Statistics
        stats: {
            totalTests: 'Total Tests',
            passRate: 'Pass Rate',
            failedTests: 'Failed Tests',
            failRate: 'fail rate',
            excellent: 'Excellent',
            good: 'Good',
            needsAttention: 'Needs Attention'
        },

        // Events
        events: {
            title: 'Events',
            newEvent: 'New Event',
            originatorId: 'Originator ID',
            targetId: 'Target ID',
            reportId: 'Report ID',
            comment: 'Comment',
            status: 'Status',
            complete: 'Complete',
            pending: 'Pending',
            createdAt: 'Created At',
            noEvents: 'No events found.'
        },

        // Voltech
        voltech: {
            import: {
                title: 'Import Voltech Data',
                fullImport: 'Full Import',
                startDate: 'Start Date',
                endDate: 'End Date'
            },
            watcher: {
                title: 'Watcher Status',
                start: 'Start Watcher',
                stop: 'Stop Watcher',
                pause: 'Pause',
                resume: 'Resume'
            }
        }
    },
    es : {
        voltech: {
            import: {
                title: 'Importar datos de Voltech'
            }
        }
    }
} as const;

// Current language - could be loaded from user preferences or browser settings
let currentLanguage: Language = 'en';

export function setLanguage(lang: Language) {
    currentLanguage = lang;
}

export function getLanguage(): Language {
    return currentLanguage;
}

// Helper function to get translated string with type safety
export function t(key: string): string {
    const keys = key.split('.');
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    let value: any = strings[currentLanguage];

    for (const k of keys) {
        if (value && typeof value === 'object' && k in value) {
            value = value[k];
        } else {
            console.warn(`Translation key not found: ${key}`);
            return key;
        }
    }

    return typeof value === 'string' ? value : key;
}

// Type-safe translation function with autocomplete
type NestedKeyOf<T> = T extends object
    ? {
        [K in keyof T]: K extends string
        ? T[K] extends object
        ? `${K}` | `${K}.${NestedKeyOf<T[K]>}`
        : `${K}`
        : never;
    }[keyof T]
    : never;

type TranslationKey = NestedKeyOf<typeof strings.en>;

export function tr(key: TranslationKey): string {
    return t(key);
}
