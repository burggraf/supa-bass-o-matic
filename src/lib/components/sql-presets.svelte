<!-- src/lib/components/sql-presets.svelte -->
<script lang="ts">
import presets from '$lib/data/sql-presets.json';
import { Button } from '$lib/components/ui/button';
import * as Dialog from '$lib/components/ui/dialog';
import * as Accordion from '$lib/components/ui/accordion';

let { onselect } = $props<{
  onselect: (sql: string) => void;
}>();

let isOpen = $state(false);

function handlePresetSelect(sql: string) {
  onselect(sql);
  isOpen = false;
}

const categories = {
  presets: { title: 'SQL Presets', items: presets.presets },
  schema: { title: 'Schema Queries', items: presets.schema }
};
</script>

<div class="flex-1">
  <Button variant="outline" onclick={() => isOpen = true} class="w-full justify-start">
    Choose a SQL preset
  </Button>

  <Dialog.Dialog bind:open={isOpen}>
    <Dialog.DialogContent class="max-w-4xl">
      <Dialog.DialogHeader>
        <Dialog.DialogTitle>SQL Presets</Dialog.DialogTitle>
        <Dialog.DialogDescription>
          Choose from our collection of pre-written SQL queries
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
                      <Accordion.Trigger>
                        <div class="flex items-center justify-between w-full">
                          <span class="font-medium">{item.title}</span>
                          <Button 
                            variant="ghost" 
                            size="sm"
                            onclick={(e) => {
                              e.stopPropagation();
                              handlePresetSelect(item.sql);
                            }}
                            class="ml-2"
                          >
                            Select
                          </Button>
                        </div>
                      </Accordion.Trigger>
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

      <Dialog.DialogFooter>
        <Button variant="outline" onclick={() => isOpen = false}>
          Cancel
        </Button>
      </Dialog.DialogFooter>
    </Dialog.DialogContent>
  </Dialog.Dialog>
</div>
