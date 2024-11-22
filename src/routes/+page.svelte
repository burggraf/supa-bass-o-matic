<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
    import { onMount } from 'svelte';
    import { Input } from "$lib/components/ui/input";
    import { Button } from "$lib/components/ui/button";
    import { Textarea } from "$lib/components/ui/textarea";
    import { Label } from "$lib/components/ui/label";
    import * as Table from "$lib/components/ui/table";
    import { Root as Select, Content as SelectContent, Item as SelectItem, Trigger as SelectTrigger } from "$lib/components/ui/select";

    let connectionString = '';
    let connectionStrings: string[] = [];
    let selectedConnectionString = '';
    let sqlQuery = '';
    let queryResult: { columns: string[], rows: string[][] } | null = null;
    let error: string | null = null;
    let debugOutput: string[] = [];
    let isLoading = false;

    onMount(() => {
        console.log('Component mounted');
        loadConnectionStrings();
    });

    function loadConnectionStrings() {
        console.log('Loading connection strings');
        const stored = localStorage.getItem('connectionStrings');
        if (stored) {
            connectionStrings = JSON.parse(stored);
            console.log('Loaded connection strings:', connectionStrings);
            if (connectionStrings.length > 0) {
                selectedConnectionString = connectionStrings[0];
                connectionString = selectedConnectionString;
                console.log('Selected first connection string:', selectedConnectionString);
            }
        } else {
            console.log('No stored connection strings found');
        }
    }

    function saveConnectionStrings() {
        localStorage.setItem('connectionStrings', JSON.stringify(connectionStrings));
    }

    function addCurrentConnection() {
        if (connectionString && !connectionStrings.includes(connectionString)) {
            connectionStrings = [...connectionStrings, connectionString];
            selectedConnectionString = connectionString;
            saveConnectionStrings();
            addDebug(`Added new connection string: ${connectionString}`);
        }
    }

    function deleteConnection(conn: string) {
        connectionStrings = connectionStrings.filter(c => c !== conn);
        if (selectedConnectionString === conn) {
            selectedConnectionString = connectionStrings[0] || '';
            connectionString = selectedConnectionString;
        }
        saveConnectionStrings();
        addDebug(`Deleted connection string: ${conn}`);
    }

    function handleConnectionChange(value: string) {
        console.log('Connection changed:', value);
        selectedConnectionString = value;
        connectionString = value;
        addDebug(`Selected connection string: ${value}`);
    }

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

    $: console.log('selectedConnectionString changed:', selectedConnectionString);
</script>

<div class="min-h-screen bg-white text-gray-900">
    <main class="container mx-auto py-8 px-4">
        <h1 class="text-3xl font-bold mb-6">Supa Bass-a-matic PostgreSQL Query Tool</h1>
        
        <div class="space-y-6">
            <div class="space-y-2">
                <Label for="connection-string">Connection String</Label>
                <div class="flex gap-2">
                    <div class="flex-1">
                        <Input 
                            id="connection-string"
                            type="text" 
                            bind:value={connectionString} 
                            placeholder="postgres://user:pass@host:5432/database"
                        />
                    </div>
                    <Button 
                        onclick={addCurrentConnection}
                        variant="outline"
                        class="whitespace-nowrap"
                    >
                        Save Connection
                    </Button>
                </div>
            </div>

            <div class="space-y-2">
                <Label>Saved Connections</Label>
                <div class="flex gap-2">
                    <div class="flex-1">
                        <Select type="single" value={selectedConnectionString} onValueChange={handleConnectionChange}>
                            <SelectTrigger class="w-full">
                                <span class="truncate">
                                    {selectedConnectionString || "Select a saved connection"}
                                </span>
                            </SelectTrigger>
                            <SelectContent>
                                {#each connectionStrings as conn}
                                    <SelectItem value={conn}>
                                        {conn}
                                    </SelectItem>
                                {/each}
                            </SelectContent>
                        </Select>
                    </div>
                    {#if selectedConnectionString}
                        <Button 
                            onclick={() => deleteConnection(selectedConnectionString)}
                            variant="destructive"
                            class="whitespace-nowrap"
                        >
                            Delete Connection
                        </Button>
                    {/if}
                </div>
            </div>

            <div class="space-y-2">
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
                class="bg-blue-500 hover:bg-blue-600 text-white"
            >
                {#if isLoading}
                    Executing...
                {:else}
                    Execute Query
                {/if}
            </Button>
        </div>

        {#if error}
            <div class="p-4 mt-6 border border-red-500 rounded-md text-red-500">
                Error: {error}
            </div>
        {/if}

        {#if queryResult}
            <div class="mt-6">
                <h2 class="text-2xl font-bold mb-4">Query Results</h2>
                <div class="relative overflow-x-auto">
                    <div class="rounded-md border border-gray-200">
                        <Table.Root>
                            <Table.Header>
                                <Table.Row>
                                    {#each queryResult.columns as column}
                                        <Table.Head class="bg-gray-50 p-4 text-left font-medium text-gray-900">
                                            {column}
                                        </Table.Head>
                                    {/each}
                                </Table.Row>
                            </Table.Header>
                            <Table.Body>
                                {#each queryResult.rows as row}
                                    <Table.Row>
                                        {#each row as cell}
                                            <Table.Cell class="p-4 border-t border-gray-200">
                                                {cell}
                                            </Table.Cell>
                                        {/each}
                                    </Table.Row>
                                {/each}
                            </Table.Body>
                        </Table.Root>
                    </div>
                </div>
            </div>
        {/if}

        <div class="mt-6 p-4 bg-gray-50 rounded-md">
            <h3 class="text-lg font-semibold mb-2">Debug Output ({debugOutput.length} messages)</h3>
            <div class="max-h-[200px] overflow-y-auto border border-gray-200 rounded-md p-2 bg-white">
                {#each debugOutput as message}
                    <div class="font-mono text-sm whitespace-pre-wrap mb-1">{message}</div>
                {/each}
            </div>
        </div>
    </main>
</div>