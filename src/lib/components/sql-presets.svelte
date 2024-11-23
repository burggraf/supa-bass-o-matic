<!-- src/lib/components/sql-presets.svelte -->
<script lang="ts">
import presets from '$lib/data/sql-presets.json';
import * as Select from '$lib/components/ui/select';
import { createEventDispatcher } from 'svelte';

const dispatch = createEventDispatcher();

let selectedPresetId = $state('');

function handlePresetSelect(value: string) {
  selectedPresetId = value;
  const preset = presets.presets.find(p => p.id === value);
  if (preset) {
    dispatch('select', preset.sql);
  }
}
</script>

<div class="flex-1">
  <Select.Select 
    type="single"
    value={selectedPresetId} 
    onValueChange={handlePresetSelect}
  >
    <Select.Trigger class="w-full">
      <span class="truncate">
        {presets.presets.find(p => p.id === selectedPresetId)?.title || "Choose a SQL preset"}
      </span>
    </Select.Trigger>
    <Select.Content>
      {#each presets.presets as preset}
        <Select.Item value={preset.id}>
          <div class="flex flex-col">
            <span>{preset.title}</span>
            <span class="text-sm text-muted-foreground">{preset.description}</span>
          </div>
        </Select.Item>
      {/each}
    </Select.Content>
  </Select.Select>
</div>
