<script>
    import { invoke } from '@tauri-apps/api/tauri'

    let name = ''
    let greetMsg = ''

    async function greet() {
        greetMsg = await invoke('greet', { name })
    }

    let result = ''
    async function createBeneficiary() {
        result = await invoke('create_transaction')
    }
</script>

<div>
    <input id="greet-input" placeholder="Enter a name..." bind:value="{name}" />
    <button on:click="{() => greet()}">Greet</button>
    <p>{greetMsg}</p>
    
    <button on:click="{() => createBeneficiary()}">Create Transaction For</button>
    <p>{result}</p>
</div>
