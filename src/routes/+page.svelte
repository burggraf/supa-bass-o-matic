<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
    import { Input } from "$lib/components/ui/input";
    import { Button } from "$lib/components/ui/button";
    import { Textarea } from "$lib/components/ui/textarea";
    import { Label } from "$lib/components/ui/label";
    import * as Table from "$lib/components/ui/table";
    import * as Select from "$lib/components/ui/select";
    import * as Dialog from "$lib/components/ui/dialog";
    import { Trash2, Check, Plus, Pencil } from "lucide-svelte";
    import SqlPresets from "$lib/components/sql-presets.svelte";

    interface Connection {
        title: string;
        url: string;
    }

    let connectionString = $state('');
    let connectionTitle = $state('');
    let connections = $state<Connection[]>([]);
    let selectedConnection = $state<Connection | null>(null);
    let sqlQuery = $state('');
    let queryResult = $state<{ columns: string[], rows: string[][] } | null>(null);
    let error = $state<string | null>(null);
    let debugOutput = $state<string[]>([]);
    let isLoading = $state(false);
    let isInitialized = $state(false);
    let isDialogOpen = $state(false);
    let isEditing = $state(false);
    let selectRef: { close: () => void } | null = $state(null);

    // Load connections only once on mount
    $effect(() => {
        if (!isInitialized) {
            console.log('Component mounted');
            loadConnections();
            isInitialized = true;
        }
    });

    // Clear fields when dialog opens for new connection
    $effect(() => {
        if (isDialogOpen && !isEditing) {
            connectionString = '';
            connectionTitle = '';
        }
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
            
            if (isEditing) {
                // Update existing connection
                connections = connections.map(conn => 
                    conn.url === selectedConnection?.url ? newConnection : conn
                );
                selectedConnection = newConnection;
            } else {
                // Add new connection
                connections = [...connections, newConnection];
                selectedConnection = newConnection;
            }
            
            saveConnections();
            addDebug(`${isEditing ? 'Updated' : 'Added new'} connection: ${connectionTitle}`);
            isEditing = false;
        }
    }

    function editConnection(conn: Connection) {
        selectRef?.close();
        connectionTitle = conn.title;
        connectionString = conn.url;
        isEditing = true;
        isDialogOpen = true;
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

    async function executeQuery() {
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

    $effect(() => {
        // console.log('selectedConnection changed:', selectedConnection);
    });
</script>

<div class="min-h-screen bg-white text-gray-900">
    <main class="container mx-auto py-8 px-4">
        <div class="flex justify-between items-center mb-6">
            <h1 class="text-3xl font-bold">Supa Bass-a-matic PostgreSQL Query Tool</h1>
            <Button onclick={() => isDialogOpen = true}>
                <Plus class="w-4 h-4 mr-2" />
                Add Connection
            </Button>
        </div>
        
        <div class="space-y-4">
            {#if connections.length > 0}
                <div class="grid grid-cols-[150px_1fr] items-center gap-4">
                    <Label>Saved Connections</Label>
                    <div class="flex gap-2">
                        <Select.Select 
                            type="single" 
                            value={selectedConnection?.url} 
                            onValueChange={handleConnectionChange}
                            ref={selectRef}
                        >
                            <Select.Trigger class="w-full">
                                <span class="truncate">
                                    {selectedConnection ? selectedConnection.title : "Select a saved connection"}
                                </span>
                            </Select.Trigger>
                            <Select.Content>
                                {#each connections as conn}
                                    <Select.Item value={conn.url}>
                                        <div class="flex items-center justify-between w-full">
                                            <span>{conn.title}</span>
                                            <div class="flex gap-2">
                                                <button
                                                    class="text-blue-500 hover:text-blue-700"
                                                    onclick={(e) => {
                                                        e.preventDefault();
                                                        editConnection(conn);
                                                    }}
                                                >
                                                    <Pencil class="w-4 h-4" />
                                                </button>
                                                <button
                                                    class="text-red-500 hover:text-red-700"
                                                    onclick={(e) => {
                                                        e.preventDefault();
                                                        deleteConnection(conn.url);
                                                    }}
                                                >
                                                    <Trash2 class="w-4 h-4" />
                                                </button>
                                            </div>
                                        </div>
                                    </Select.Item>
                                {/each}
                            </Select.Content>
                        </Select.Select>
                    </div>
                </div>
            {:else}
                <div class="text-center py-8 text-gray-500">
                    No connections added yet. Click "Add Connection" to get started.
                </div>
            {/if}

            <div class="space-y-4">
                <div class="grid grid-cols-[150px_1fr] items-center gap-4">
                    <Label>SQL Query</Label>
                    <div class="space-y-4">
                        <SqlPresets 
                            on:select={(e) => {
                                sqlQuery = e.detail;
                            }}
                            hasConnection={!!selectedConnection}
                        />
                        <Textarea
                            bind:value={sqlQuery}
                            placeholder="Enter your SQL query here..."
                            class="min-h-[200px]"
                        />
                        <Button 
                            onclick={executeQuery} 
                            disabled={isLoading || !selectedConnection}
                            class="w-full"
                        >
                            Execute Query
                        </Button>
                    </div>
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
            </div>

            <Dialog.Dialog bind:open={isDialogOpen}>
                <Dialog.DialogContent class="z-[100]">
                    <Dialog.DialogHeader>
                        <Dialog.DialogTitle>{isEditing ? 'Edit' : 'Add'} Connection</Dialog.DialogTitle>
                        <Dialog.DialogDescription>
                            {isEditing ? 'Update the connection details below.' : 'Enter the connection details below.'}
                        </Dialog.DialogDescription>
                    </Dialog.DialogHeader>

                    <div class="grid gap-4 py-4">
                        <div class="grid gap-2">
                            <Label for="title">Connection Title</Label>
                            <Input
                                id="title"
                                bind:value={connectionTitle}
                                placeholder="My Database"
                            />
                        </div>
                        <div class="grid gap-2">
                            <Label for="url">Connection String</Label>
                            <Input
                                id="url"
                                bind:value={connectionString}
                                placeholder="postgres://user:pass@localhost:5432/db"
                            />
                        </div>
                    </div>

                    <Dialog.DialogFooter>
                        <Button variant="outline" onclick={() => {
                            isDialogOpen = false;
                            isEditing = false;
                        }}>
                            Cancel
                        </Button>
                        <Button 
                            disabled={!connectionString || !connectionTitle}
                            onclick={() => {
                                addCurrentConnection();
                                isDialogOpen = false;
                            }}
                        >
                            <Check class="w-4 h-4 mr-2" />
                            {isEditing ? 'Update' : 'Save'} Connection
                        </Button>
                    </Dialog.DialogFooter>
                </Dialog.DialogContent>
            </Dialog.Dialog>
        </main>
    </div>