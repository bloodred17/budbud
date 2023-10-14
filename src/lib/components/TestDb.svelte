<script lang="ts">
    import {invoke} from "@tauri-apps/api/tauri";

    let input = '';
    let connection_output = '';

    async function connect() {
        connection_output = await invoke('establish_connection');
    }

    async function createEntry() {
        connection_output = await invoke('create_entry');
    }

    async function deleteEntry() {
        connection_output = await invoke('delete_entry', { table: 'transaction_source', id: input });
    }

    async function read() {
        connection_output = await invoke('read', { table: 'transaction_source' });
        if (connection_output?.includes("data")) {
            connection_output = JSON.parse(connection_output);
        }
    }

    async function update() {
        connection_output = await invoke('update');
    }
</script>

<div class="flex">
    <button class="btn btn-success" on:click={() => connect()}>Connect</button>
    <button class="btn btn-primary" on:click={() => createEntry()}>Create</button>
    <button class="btn btn-info" on:click={() => read()}>Read</button>
    <button class="btn btn-warning" on:click={() => update()}>Update</button>
    <button class="btn btn-error" on:click={() => deleteEntry()}>Delete</button>
</div>

<div>
    <h1>Result</h1>

    {#if typeof connection_output === "string"}
        <p>{connection_output}</p>
    {:else}
    <pre>
        {JSON.stringify(connection_output, undefined, 2)}
    </pre>
    {/if}

    <input type="text" placeholder="Type here" class="input input-bordered input-sm w-full max-w-xs" bind:value={input} />
    <pre>{input}</pre>
</div>