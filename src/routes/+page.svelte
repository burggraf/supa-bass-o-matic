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
    let queryResults = $state<Array<{
        title: string;
        description: string;
        result: { columns: string[], rows: string[][] };
    }>>([]);
    let error = $state<string | null>(null);
    let debugOutput = $state<string[]>([]);
    let isLoading = $state(false);
    let isInitialized = $state(false);
    let isDialogOpen = $state(false);
    let isEditing = $state(false);
    let isDebugVisible = $state(false);
    let selectRef: { close: () => void } | null = $state(null);
    let indexAdvisorStatus = $state('unknown');

    // Format cell value based on column type and content
    function formatCellValue(value: any, columnName: string): string {
        if (value === null) return 'null';
        
        // Handle special column types
        switch (columnName.toLowerCase()) {
            case 'transactionid':
                return value.toString();
            default:
                return value.toString();
        }
    }

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
            
            // Load the last selected connection
            const lastSelectedUrl = localStorage.getItem('selectedConnectionUrl');
            if (lastSelectedUrl) {
                const lastSelected = connections.find(conn => conn.url === lastSelectedUrl);
                if (lastSelected) {
                    selectedConnection = lastSelected;
                    connectionString = lastSelected.url;
                    connectionTitle = lastSelected.title;
                    return;
                }
            }
            
            // Fallback to first connection if no saved selection or saved selection not found
            if (connections.length > 0) {
                selectedConnection = connections[0];
                connectionString = selectedConnection.url;
                connectionTitle = selectedConnection.title;
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
        const conn = connections.find(c => c.url === value);
        if (conn) {
            selectedConnection = conn;
            connectionString = conn.url;
            connectionTitle = conn.title;
            indexAdvisorStatus = 'unknown';
            // Save the selected connection URL
            localStorage.setItem('selectedConnectionUrl', conn.url);
        }
    }

    function deleteConnection(url: string) {
        connections = connections.filter(conn => conn.url !== url);
        saveConnections();
        
        // If we deleted the selected connection, clear the selection
        if (selectedConnection?.url === url) {
            localStorage.removeItem('selectedConnectionUrl');
            if (connections.length > 0) {
                handleConnectionChange(connections[0].url);
            } else {
                selectedConnection = null;
                connectionString = '';
                connectionTitle = '';
            }
        }
        addDebug(`Deleted connection with URL: ${url}`);
    }

    function addDebug(message: string) {
        debugOutput = [...debugOutput, `${new Date().toISOString()}: ${message}`];
    }

    async function executeQuery() {
        addDebug('Button clicked');
        isLoading = true;
        error = null;
        queryResults = [];

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
            queryResults = [{ title: 'Query Result', description: '', result }];
        } catch (e) {
            const errorMessage = e instanceof Error ? e.message : String(e);
            error = errorMessage;
            queryResults = [];
            addDebug(`Error: ${errorMessage}`);
            console.error('Full error:', e);
        } finally {
            isLoading = false;
        }
    }

    async function checkIndexAdvisorStatus() {
        if (!connectionString) return;
        
        try {
            // First check if extension is installed
            const installedResult = await invoke('execute_query', {
                connectionString,
                query: "SELECT extname FROM pg_extension WHERE extname = 'index_advisor'"
            });
            
            if (installedResult.rows.length > 0) {
                indexAdvisorStatus = 'installed';
                return;
            }
            
            // If not installed, check if it's available
            const availableResult = await invoke('execute_query', {
                connectionString,
                query: "SELECT name FROM pg_available_extensions WHERE name = 'index_advisor'"
            });
            
            indexAdvisorStatus = availableResult.rows.length > 0 ? 'available' : 'not available';
        } catch (e) {
            console.error('Error checking index advisor status:', e);
            indexAdvisorStatus = 'unknown';
        }
    }

    $effect(() => {
        // console.log('selectedConnection changed:', selectedConnection);
    });
</script>

<div class="min-h-screen bg-white text-gray-900">
    <main class="container mx-auto py-8 px-4">
        <div class="grid grid-cols-3 items-center mb-6">
            <div class="justify-self-start">
                {#if selectedConnection}
                    <Button 
                        variant={indexAdvisorStatus === 'installed' ? 'default' : 
                                indexAdvisorStatus === 'available' ? 'outline' : 
                                indexAdvisorStatus === 'not available' ? 'destructive' : 'secondary'}
                        onclick={checkIndexAdvisorStatus}
                        disabled={!selectedConnection}
                    >
                        Index Advisor Status: {indexAdvisorStatus}
                    </Button>
                {/if}
            </div>
            <h1 class="text-3xl font-bold text-center justify-self-center">supa-bass-o-matic</h1>
            <div class="justify-self-end">
                <Button onclick={() => isDialogOpen = true}>
                    <Plus class="w-4 h-4 mr-2" />
                    Add Connection
                </Button>
            </div>
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
                            onselect={(e) => {
                                sqlQuery = e;
                                error = null;  // Clear previous errors
                            }}
                            onexecutemultiple={async (queries) => {
                                isLoading = true;
                                error = null;
                                queryResults = [];
                                
                                for (const query of queries) {
                                    try {
                                        const result = await invoke('execute_query', {
                                            connectionString,
                                            query: query.sql
                                        });
                                        queryResults = [...queryResults, {
                                            title: query.title,
                                            description: query.description || '',
                                            result
                                        }];
                                    } catch (err) {
                                        error = `Error executing "${query.title}": ${err}`;
                                        break;
                                    }
                                }
                                isLoading = false;
                            }}
                        />
                        <Textarea
                            bind:value={sqlQuery}
                            placeholder="Enter your SQL query here"
                            class="font-mono"
                            rows="6"
                        />
                        <div class="flex justify-end gap-4">
                            <Button 
                                disabled={isLoading || !connectionString || !sqlQuery}
                                onclick={executeQuery}
                            >
                                {#if isLoading}
                                    <span class="inline-block animate-spin mr-2">⟳</span>
                                    Executing...
                                {:else}
                                    Execute Query
                                {/if}
                            </Button>
                        </div>
                    </div>
                </div>

                {#if error}
                    <div class="mt-4 p-4 bg-red-50 border border-red-200 rounded-md text-red-700">
                        <div class="font-bold mb-2">Error executing query:</div>
                        <pre class="whitespace-pre-wrap text-sm">{error}</pre>
                        <div class="mt-2 text-sm">
                            <Button 
                                variant="outline" 
                                class="text-red-600 hover:text-red-800"
                                onclick={() => {
                                    error = null;
                                    isLoading = false;
                                }}
                            >
                                Clear Error
                            </Button>
                        </div>
                    </div>
                {/if}

                {#if queryResults.length > 0}
                    <div class="mt-4 space-y-8">
                        {#each queryResults as result}
                            <div class="border rounded-lg overflow-hidden">
                                <div class="bg-muted p-4">
                                    <h3 class="font-semibold text-lg mb-1">{result.title}</h3>
                                    {#if result.description}
                                        <p class="text-sm text-muted-foreground">{result.description}</p>
                                    {/if}
                                </div>
                                
                                <div class="overflow-x-auto">
                                    <Table.Root>
                                        <Table.Header>
                                            <Table.Row>
                                                {#each result.result.columns as column}
                                                    <Table.Head>{column}</Table.Head>
                                                {/each}
                                            </Table.Row>
                                        </Table.Header>
                                        <Table.Body>
                                            {#each result.result.rows as row}
                                                <Table.Row>
                                                    {#each row as cell, i}
                                                        <Table.Cell>
                                                            {formatCellValue(cell, result.result.columns[i])}
                                                        </Table.Cell>
                                                    {/each}
                                                </Table.Row>
                                            {/each}
                                        </Table.Body>
                                    </Table.Root>
                                </div>
                            </div>
                        {/each}
                    </div>
                {/if}

                <div class="mt-6">
                    <div class="flex items-center justify-between mb-2">
                        <Button 
                            variant="outline" 
                            size="sm"
                            onclick={() => isDebugVisible = !isDebugVisible}
                        >
                            {isDebugVisible ? 'Hide' : 'Show'} Debug Output ({debugOutput.length})
                        </Button>
                    </div>
                    {#if isDebugVisible}
                        <div class="p-4 bg-gray-50 rounded-md">
                            <div class="max-h-[200px] overflow-y-auto border border-gray-200 rounded-md p-2 bg-white">
                                {#each debugOutput as message}
                                    <div class="font-mono text-sm whitespace-pre-wrap mb-1">{message}</div>
                                {/each}
                            </div>
                        </div>
                    {/if}
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