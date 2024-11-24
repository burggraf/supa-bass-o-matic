<!-- src/lib/components/sql-presets.svelte -->
<script lang="ts">
import presets from '$lib/data/sql-presets.json';
import { Button } from '$lib/components/ui/button';
import * as Dialog from '$lib/components/ui/dialog';
import * as Table from '$lib/components/ui/table';

let { onselect } = $props<{
  onselect: (sql: string) => void;
}>();

let isOpen = $state(false);

function handlePresetSelect(sql: string) {
  onselect(sql);
  isOpen = false;
}
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
        <Table.Table>
          <Table.TableHeader class="sticky top-0 bg-background">
            <Table.TableRow>
              <Table.TableHead>Name</Table.TableHead>
              <Table.TableHead>Description</Table.TableHead>
              <Table.TableHead class="w-[100px]">Action</Table.TableHead>
            </Table.TableRow>
          </Table.TableHeader>
          <Table.TableBody>
            {#each presets.presets as preset}
              <Table.TableRow>
                <Table.TableCell class="font-medium">{preset.title}</Table.TableCell>
                <Table.TableCell>{preset.description}</Table.TableCell>
                <Table.TableCell>
                  <Button 
                    variant="ghost" 
                    size="sm"
                    onclick={() => handlePresetSelect(preset.sql)}
                  >
                    Select
                  </Button>
                </Table.TableCell>
              </Table.TableRow>
            {/each}
          </Table.TableBody>
        </Table.Table>
      </div>

      <Dialog.DialogFooter>
        <Button variant="outline" onclick={() => isOpen = false}>
          Cancel
        </Button>
      </Dialog.DialogFooter>
    </Dialog.DialogContent>
  </Dialog.Dialog>
</div>
