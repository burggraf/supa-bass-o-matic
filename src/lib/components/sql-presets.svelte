<!-- src/lib/components/sql-presets.svelte -->
<script lang="ts">
import presets from '$lib/data/sql-presets.json';
import { Button } from '$lib/components/ui/button';
import * as Dialog from '$lib/components/ui/dialog';
import * as Accordion from '$lib/components/ui/accordion';
import { Checkbox } from '$lib/components/ui/checkbox';
import { onMount } from 'svelte';

let { onselect, onexecutemultiple, connectionId } = $props<{
  onselect: (sql: string) => void;
  onexecutemultiple: (queries: Array<{ title: string, sql: string, description: string }>) => void;
  connectionId: string;
}>();

let isOpen = $state(false);
let selectedItems = $state<Array<{ id: string, title: string, sql: string }>>([]);

// Load saved selections on mount
onMount(() => {
  const savedSelections = localStorage.getItem('sql-preset-selections');
  if (savedSelections) {
    try {
      selectedItems = JSON.parse(savedSelections);
    } catch (e) {
      console.error('Error loading saved SQL preset selections:', e);
      selectedItems = [];
    }
  }
});

// Save selections whenever they change
$effect(() => {
  if (selectedItems.length === 0) {
    localStorage.removeItem('sql-preset-selections');
  } else {
    localStorage.setItem('sql-preset-selections', JSON.stringify(selectedItems));
  }
});

function isSelected(id: string) {
  return selectedItems.some(item => item.id === id);
}

function getSelectedCountForCategory(categoryItems: any[]) {
  return selectedItems.filter(selected => 
    categoryItems.some(item => item.id === selected.id)
  ).length;
}

function handleGroupSelect(categoryItems: any[], checked: boolean) {
  if (checked) {
    // Add all unselected items from the category
    const newItems = categoryItems
      .filter(item => !isSelected(item.id))
      .map(item => ({ id: item.id, title: item.title, sql: item.sql }));
    selectedItems = [...selectedItems, ...newItems];
  } else {
    // Remove all items from this category
    selectedItems = selectedItems.filter(selected => 
      !categoryItems.some(item => item.id === selected.id)
    );
  }
}

function isGroupFullySelected(categoryItems: any[]) {
  return categoryItems.every(item => isSelected(item.id));
}

function getTotalItems() {
  return Object.values(categories).reduce((total, category) => total + category.items.length, 0);
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
    // Don't clear selections after execution
  }
}

const categories = Object.entries(presets).reduce((acc, [key, category]) => {
  acc[key] = { title: category.title, items: category.items };
  return acc;
}, {} as Record<string, { title: string, items: any[] }>);
</script>

<div class="flex-1 flex gap-2">
  <Button variant="outline" onclick={() => isOpen = true} class="w-full justify-start">
    Choose SQL presets ({selectedItems.length} selected)
  </Button>
  <Button 
    variant="default" 
    onclick={handleExecuteSelected}
    disabled={selectedItems.length === 0}
  >
    Execute Selected ({selectedItems.length})
  </Button>
</div>

<Dialog.Dialog bind:open={isOpen}>
  <Dialog.DialogContent class="max-w-4xl">
    <Dialog.DialogHeader>
      <Dialog.DialogTitle>SQL Presets ({selectedItems.length} of {getTotalItems()} selected)</Dialog.DialogTitle>
      <Dialog.DialogDescription>
        Choose from predefined SQL queries to explore and analyze your data
      </Dialog.DialogDescription>
    </Dialog.DialogHeader>

    <div class="relative max-h-[60vh] overflow-auto">
      <Accordion.Root type="multiple">
        {#each Object.entries(categories) as [key, category]}
          <Accordion.Item value={key} class="border-b-0">
            <div class="flex items-center gap-2 bg-muted/50 hover:bg-muted px-4 py-2">
              <div onclick={(e) => e.stopPropagation()}>
                <Checkbox
                  checked={isGroupFullySelected(category.items)}
                  onCheckedChange={(checked: boolean) => handleGroupSelect(category.items, checked)}
                />
              </div>
              <Accordion.Trigger class="flex-1">
                <span class="font-semibold">{category.title} ({getSelectedCountForCategory(category.items)}/{category.items.length})</span>
              </Accordion.Trigger>
            </div>
            <Accordion.Content>
              <Accordion.Root type="single" class="px-2">
                {#each category.items as item}
                  <Accordion.Item value={item.id}>
                    <button onclick={(e) => e.stopPropagation()} class="flex items-center px-4 py-2">
                      <Checkbox
                        checked={isSelected(item.id)}
                        onCheckedChange={(checked: boolean) => {
                          handlePresetSelect(item.id, item.title, item.sql, checked);
                        }}
                        class="mr-2"
                      />
                      <Accordion.Trigger class="flex-1 text-left">
                        <span class="font-medium">{item.title}</span>
                      </Accordion.Trigger>
                    </button>
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
