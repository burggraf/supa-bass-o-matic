<!-- src/lib/components/sql-presets.svelte -->
<script lang="ts">
import presets from '$lib/data/sql-presets.json';
import { Button } from '$lib/components/ui/button';
import * as Dialog from '$lib/components/ui/dialog';
import * as Accordion from '$lib/components/ui/accordion';
import { Checkbox } from '$lib/components/ui/checkbox';

let { onselect, onexecutemultiple } = $props<{
  onselect: (sql: string) => void;
  onexecutemultiple: (queries: Array<{ title: string, sql: string, description: string }>) => void;
}>();

let isOpen = $state(false);
let selectedItems = $state<Array<{ id: string, title: string, sql: string }>>([]);

function isSelected(id: string) {
  return selectedItems.some(item => item.id === id);
}

function handlePresetSelect(id: string, title: string, sql: string, checked: boolean) {
  console.log('handlePresetSelect', { id, title, checked, currentSelected: selectedItems });
  
  if (!checked) {
    selectedItems = selectedItems.filter(item => item.id !== id);
  } else {
    selectedItems = [...selectedItems, { id, title, sql }];
  }
  
  console.log('After update:', selectedItems);
}

function handleExecuteSelected() {
  console.log('handleExecuteSelected', selectedItems);
  if (selectedItems.length > 0) {
    onexecutemultiple(selectedItems.map(({ id, title, sql }) => {
      // Find the original item to get its description
      const category = Object.values(categories).find(cat => 
        cat.items.some(item => item.id === id)
      );
      const originalItem = category?.items.find(item => item.id === id);
      return { 
        title, 
        sql,
        description: originalItem?.description || ''
      };
    }));
    isOpen = false;
    selectedItems = [];
  }
}

const categories = {
  presets: { title: 'SQL Presets', items: presets.presets },
  schema: { title: 'Schema Queries', items: presets.schema }
};
</script>

<div class="flex-1">
  <Button variant="outline" onclick={() => isOpen = true} class="w-full justify-start">
    Choose SQL presets ({selectedItems.length} selected)
  </Button>

  <Dialog.Dialog bind:open={isOpen}>
    <Dialog.DialogContent class="max-w-4xl">
      <Dialog.DialogHeader>
        <Dialog.DialogTitle>SQL Presets</Dialog.DialogTitle>
        <Dialog.DialogDescription>
          Select one or more SQL queries to execute
        </Dialog.DialogDescription>
      </Dialog.DialogHeader>

      <div class="relative max-h-[60vh] overflow-auto">
        <Accordion.Root type="multiple">
          {#each Object.entries(categories) as [key, category]}
            <Accordion.Item value={key} class="border-b-0">
              <Accordion.Trigger class="bg-muted/50 hover:bg-muted">
                <span class="font-semibold">{category.title}</span>
              </Accordion.Trigger>
              <Accordion.Content>
                <Accordion.Root type="single" class="px-2">
                  {#each category.items as item}
                    <Accordion.Item value={item.id}>
                      <div class="flex items-center px-4 py-2">
                        <div onclick={(e) => e.stopPropagation()}>
                          <Checkbox
                            checked={isSelected(item.id)}
                            onCheckedChange={(checked: boolean) => {
                              handlePresetSelect(item.id, item.title, item.sql, checked);
                            }}
                            class="mr-2"
                          />
                        </div>
                        <Accordion.Trigger class="flex-1 text-left">
                          <span class="font-medium">{item.title}</span>
                        </Accordion.Trigger>
                      </div>
                      <Accordion.Content class="-mt-4">
                        <div class="pb-4">
                          <p class="text-sm text-muted-foreground mb-2">{item.description}</p>
                          <pre class="text-xs bg-muted p-2 rounded-md whitespace-pre-wrap">{@html item.sql.trim().replace(/\\n/g, '\n')}</pre>
                        </div>
                      </Accordion.Content>
                    </Accordion.Item>
                  {/each}
                </Accordion.Root>
              </Accordion.Content>
            </Accordion.Item>
          {/each}
        </Accordion.Root>
      </div>

      <Dialog.DialogFooter class="flex justify-between">
        <div class="text-sm text-muted-foreground">
          {selectedItems.length} queries selected
        </div>
        <div class="space-x-2">
          <Button variant="outline" onclick={() => {
            isOpen = false;
            selectedItems = [];
          }}>
            Cancel
          </Button>
          <Button 
            variant="default" 
            onclick={handleExecuteSelected}
            disabled={selectedItems.length === 0}
          >
            Execute Selected ({selectedItems.length})
          </Button>
        </div>
      </Dialog.DialogFooter>
    </Dialog.DialogContent>
  </Dialog.Dialog>
</div>
