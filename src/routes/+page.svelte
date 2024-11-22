<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
    import { onMount } from 'svelte';
    import { Input } from "$lib/components/ui/input";
    import { Button } from "$lib/components/ui/button";
    import { Textarea } from "$lib/components/ui/textarea";
    import { Label } from "$lib/components/ui/label";
    import * as Table from "$lib/components/ui/table";
    import * as Select from "$lib/components/ui/select/index.js";
    import { effect } from 'svelte';

    interface Connection {
        title: string;
        url: string;
    }

    let connectionString = '';
    let connectionTitle = '';
    let connections: Connection[] = [];
    let selectedConnection: Connection | null = null;
    let sqlQuery = '';
    let queryResult: { columns: string[], rows: string[][] } | null = null;
    let error: string | null = null;
    let debugOutput: string[] = [];
    let isLoading = false;

    onMount(() => {
        console.log('Component mounted');
        loadConnections();
    });

    function loadConnections() {
        console.log('Loading connections');
        const stored = localStorage.getItem('connections');
        if (stored) {
            connections = JSON.parse(stored);
            console.log('Loaded connections:', connections);
            if (connections.length > 0) {
                selectedConnection = connections[0];
                connectionString = selectedConnection.url;
                connectionTitle = selectedConnection.title;
                console.log('Selected first connection:', selectedConnection);
            }
        } else {
            console.log('No stored connections found');
        }
    }

    function saveConnections() {
        localStorage.setItem('connections', JSON.stringify(connections));
    }

    function addCurrentConnection() {
        if (connectionString && connectionTitle) {
            const newConnection = {
                title: connectionTitle,
                url: connectionString
            };
            
            // Check if connection with same URL already exists
            const existingIndex = connections.findIndex(conn => conn.url === connectionString);
            if (existingIndex === -1) {
                connections = [...connections, newConnection];
                selectedConnection = newConnection;
                saveConnections();
                addDebug(`Added new connection: ${connectionTitle} (${connectionString})`);
            }
            // Clear the input fields
            connectionTitle = '';
            connectionString = '';
        }
    }

    function handleConnectionChange(value: string) {
        const selected = connections.find(conn => conn.url === value);
        if (selected) {
            selectedConnection = selected;
            connectionString = selected.url;
            connectionTitle = selected.title;
        }
    }

    function deleteConnection(url: string) {
        connections = connections.filter(conn => conn.url !== url);
        if (selectedConnection?.url === url) {
            selectedConnection = connections[0] || null;
            connectionString = selectedConnection?.url || '';
            connectionTitle = selectedConnection?.title || '';
        }
        saveConnections();
        addDebug(`Deleted connection with URL: ${url}`);
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

    effect(() => {
        console.log('selectedConnection changed:', selectedConnection);
    });

</script>

<div class="min-h-screen bg-white text-gray-900">
    <main class="container mx-auto py-8 px-4">
        <h1 class="text-3xl font-bold mb-6">Supa Bass-a-matic PostgreSQL Query Tool</h1>
        
        <div class="space-y-6">
            <div class="space-y-2">
                <Label for="connection-title">Connection Title</Label>
                <div class="flex gap-2">
                    <div class="flex-1">
                        <Input 
                            id="connection-title"
                            type="text" 
                            bind:value={connectionTitle} 
                            placeholder="My Database Connection"
                        />
                    </div>
                </div>

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
                        disabled={!connectionString || !connectionTitle}
                    >
                        Save Connection
                    </Button>
                </div>
            </div>

            <div class="space-y-2">
                <Label>Saved Connections</Label>
                <div class="flex gap-2">
                    <div class="flex-1">
                        <Select.Select type="single" value={selectedConnection?.url} onValueChange={handleConnectionChange}>
                            <Select.Trigger class="w-full">
                                <span class="truncate">
                                    {selectedConnection ? `${selectedConnection.title} (${selectedConnection.url})` : "Select a saved connection"}
                                </span>
                            </Select.Trigger>
                            <Select.Content>
                                {#each connections as conn}
                                    <Select.Item value={conn.url}>
                                        {conn.title} ({conn.url})
                                    </Select.Item>
                                {/each}
                            </Select.Content>
                        </Select.Select>
                    </div>
                    {#if selectedConnection}
                        <Button 
                            onclick={() => deleteConnection(selectedConnection.url)}
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