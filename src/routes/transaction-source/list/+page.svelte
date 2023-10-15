<script lang="ts">
    import {goto} from "$app/navigation";
    import NavigationButtons from "$lib/components/NavigationButtons.svelte";
    import CreationButtons from "$lib/components/CreationButtons.svelte";
    import MainLayout from "$lib/layouts/MainLayout.svelte";
    import AcceptCancelButtons from "$lib/components/AcceptCancelButtons.svelte";
    import {onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/tauri";

    let tableData = [];
    onMount(() => {
        invoke<string>('list_transaction_sources')
            .then((response: string) => {
                console.log(JSON.parse(response));
                tableData = JSON.parse(response).data;
            })
    })
</script>

<MainLayout>
    <div slot="header" class="flex">
        <div class="flex">
            <NavigationButtons disableBack={true} disableForward={true}/>
        </div>
        <div class="px-2 flex justify-center flex-1">
            <h1 class="text-xl">Transaction Sources</h1>
        </div>
        <div class="flex">
            <div class="invisible">
                <AcceptCancelButtons on:accept={async () => await accept()} on:cancel={() => goto('/')} />
            </div>
        </div>
    </div>

    <div slot="body" class="px-1">

        <div class="overflow-x-auto">
            <table class="table">
                <!-- head -->
                <thead>
                    <tr>
                        <th></th>
                        <th>Name</th>
                        <th>Transaction Type</th>
                        <th></th>
                    </tr>
                </thead>
                <tbody>

                {#each tableData as row, index (row?.id?.id?.String)}
                    <tr>
                        <th>{index}</th>
                        <td>{row?.name}</td>
                        <td>{row?.transaction_type}</td>
                        <td>
                            <div class="join">
                                <!-- Edit-->
                                <button class="btn btn-xs join-item">
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L10.582 16.07a4.5 4.5 0 01-1.897 1.13L6 18l.8-2.685a4.5 4.5 0 011.13-1.897l8.932-8.931zm0 0L19.5 7.125M18 14v4.75A2.25 2.25 0 0115.75 21H5.25A2.25 2.25 0 013 18.75V8.25A2.25 2.25 0 015.25 6H10" />
                                    </svg>
                                </button>
                                <!-- Delete-->
                                <button class="btn btn-xs join-item">
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-4 h-4">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                                    </svg>
                                </button>
                            </div>
                        </td>
                    </tr>
                {/each}
                </tbody>
            </table>
        </div>

    </div>

    <CreationButtons slot="footer"/>
</MainLayout>