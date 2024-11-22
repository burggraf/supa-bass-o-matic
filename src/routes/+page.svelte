<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
    import { onMount } from 'svelte';
    import { Input } from "$lib/components/ui/input";
    import { Button } from "$lib/components/ui/button";
    import { Textarea } from "$lib/components/ui/textarea";
    import { Label } from "$lib/components/ui/label";
    import * as Table from "$lib/components/ui/table";

    let connectionString = '';
    let sqlQuery = '';
    let queryResult: { columns: string[], rows: string[][] } | null = null;
    let error: string | null = null;
    let debugOutput: string[] = [];
    let isLoading = false;

    onMount(() => {
        addDebug('Component mounted');
    });

    function addDebug(message: string) {
        debugOutput = [...debugOutput, `${new Date().toISOString()}: ${message}`];
    }

    async function handleClick() {
        addDebug('Button clicked');
        isLoading = true;
        error = null;

        try {
            if (!connectionString || !sqlQuery) {
                throw new Error('Please provide both connection string and SQL query');
            }

            addDebug(`Executing query with connection: ${connectionString}`);
            addDebug(`Query: ${sqlQuery}`);

            const result = await invoke('execute_query', { 
                connectionString,
                query: sqlQuery
            });

            addDebug('Query executed successfully');
            queryResult = result as typeof queryResult;
            addDebug(`Results received: ${JSON.stringify(result)}`);
        } catch (e) {
            const errorMessage = e as string;
            error = errorMessage;
            queryResult = null;
            addDebug(`Error: ${errorMessage}`);
        } finally {
            isLoading = false;
        }
    }
</script>

<main class="container">
    <h1>Supa Bass-a-matic PostgreSQL Query Tool</h1>
    
    <div class="query-form">
        <div class="input-group">
            <Label for="connection-string">Connection String</Label>
            <Input 
                id="connection-string"
                type="text" 
                bind:value={connectionString} 
                placeholder="postgres://user:pass@host:5432/database"
            />
        </div>

        <div class="input-group">
            <Label for="sql-query">SQL Query</Label>
            <Textarea 
                id="sql-query"
                bind:value={sqlQuery} 
                placeholder="SELECT * FROM your_table"
                class="min-h-[100px]"
            />
        </div>

        <Button 
            onclick={handleClick}
            disabled={isLoading}
            variant="default"
        >
            {#if isLoading}
                Executing...
            {:else}
                Execute Query
            {/if}
        </Button>
    </div>

    {#if error}
        <div class="error">
            Error: {error}
        </div>
    {/if}

    {#if queryResult}
        <div class="results">
            <h2>Query Results</h2>
            <div class="relative">
                <div class="rounded-md border">
                    <Table.Root>
                        <Table.Header>
                            <Table.Row>
                                {#each queryResult.columns as column}
                                    <Table.Head>{column}</Table.Head>
                                {/each}
                            </Table.Row>
                        </Table.Header>
                        <Table.Body>
                            {#each queryResult.rows as row}
                                <Table.Row>
                                    {#each row as cell}
                                        <Table.Cell>{cell}</Table.Cell>
                                    {/each}
                                </Table.Row>
                            {/each}
                        </Table.Body>
                    </Table.Root>
                </div>
            </div>
        </div>
    {/if}

    <div class="debug-output">
        <h3>Debug Output ({debugOutput.length} messages)</h3>
        <div class="debug-container">
            {#each debugOutput as message}
                <div class="debug-line">{message}</div>
            {/each}
        </div>
    </div>
</main>

<style>
    .container {
        margin: 0 auto;
        padding: 2rem;
        max-width: 1200px;
    }

    .query-form {
        margin-bottom: 2rem;
    }

    .input-group {
        margin-bottom: 1rem;
    }

    .error {
        color: red;
        margin: 1rem 0;
        padding: 1rem;
        border: 1px solid red;
        border-radius: 4px;
    }

    .results {
        margin-top: 2rem;
    }

    .debug-output {
        margin-top: 2rem;
        padding: 1rem;
        background-color: #f8f9fa;
        border-radius: 4px;
    }

    .debug-container {
        max-height: 200px;
        overflow-y: auto;
        border: 1px solid #ddd;
        padding: 0.5rem;
        border-radius: 4px;
    }

    .debug-line {
        font-family: monospace;
        white-space: pre-wrap;
        margin-bottom: 0.25rem;
    }
</style>