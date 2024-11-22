<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
    import { onMount } from 'svelte';

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
            <label for="connection-string">Connection String:</label>
            <input 
                id="connection-string"
                type="text" 
                bind:value={connectionString} 
                placeholder="postgres://user:pass@host:5432/database"
                class="input"
            />
        </div>

        <div class="input-group">
            <label for="sql-query">SQL Query:</label>
            <textarea 
                id="sql-query"
                bind:value={sqlQuery} 
                placeholder="SELECT * FROM your_table"
                class="textarea"
            ></textarea>
        </div>

        <button 
            class="execute-button" 
            on:click={handleClick}
            disabled={isLoading}
        >
            {#if isLoading}
                Executing...
            {:else}
                Execute Query
            {/if}
        </button>
    </div>

    {#if error}
        <div class="error">
            Error: {error}
        </div>
    {/if}

    {#if queryResult}
        <div class="results">
            <h2>Query Results</h2>
            <div class="table-container">
                <table>
                    <thead>
                        <tr>
                            {#each queryResult.columns as column}
                                <th>{column}</th>
                            {/each}
                        </tr>
                    </thead>
                    <tbody>
                        {#each queryResult.rows as row}
                            <tr>
                                {#each row as cell}
                                    <td>{cell}</td>
                                {/each}
                            </tr>
                        {/each}
                    </tbody>
                </table>
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
        display: flex;
        flex-direction: column;
        gap: 1rem;
        margin: 2rem 0;
    }

    .input-group {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    label {
        font-weight: bold;
    }

    .input {
        padding: 0.5rem;
        border: 1px solid #ccc;
        border-radius: 4px;
        font-size: 1rem;
        width: 100%;
    }

    .textarea {
        padding: 0.5rem;
        border: 1px solid #ccc;
        border-radius: 4px;
        font-size: 1rem;
        width: 100%;
        min-height: 150px;
        font-family: monospace;
    }

    .execute-button {
        padding: 0.5rem 1rem;
        background-color: #007bff;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        font-size: 1rem;
        transition: background-color 0.2s;
    }

    .execute-button:hover {
        background-color: #0056b3;
    }

    .execute-button:disabled {
        background-color: #ccc;
        cursor: not-allowed;
    }

    .error {
        color: red;
        margin: 1rem 0;
        padding: 1rem;
        background-color: #ffebee;
        border-radius: 4px;
    }

    .results {
        margin-top: 2rem;
    }

    .table-container {
        overflow-x: auto;
    }

    table {
        width: 100%;
        border-collapse: collapse;
        margin-top: 1rem;
    }

    th, td {
        border: 1px solid #ddd;
        padding: 0.5rem;
        text-align: left;
    }

    th {
        background-color: #f5f5f5;
    }

    tr:nth-child(even) {
        background-color: #f9f9f9;
    }

    .debug-output {
        margin-top: 2rem;
        padding: 1rem;
        background-color: #f8f9fa;
        border-radius: 4px;
    }

    .debug-container {
        max-height: 300px;
        overflow-y: auto;
        font-family: monospace;
        font-size: 0.9rem;
        background-color: #2b2b2b;
        color: #ffffff;
        padding: 1rem;
        border-radius: 4px;
    }

    .debug-line {
        padding: 0.25rem 0;
        border-bottom: 1px solid #3d3d3d;
        white-space: pre-wrap;
        word-break: break-all;
    }

    .debug-line:last-child {
        border-bottom: none;
    }
</style>