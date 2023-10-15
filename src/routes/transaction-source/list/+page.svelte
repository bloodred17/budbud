<script>
import {goto} from "$app/navigation";
import NavigationButtons from "$lib/components/NavigationButtons.svelte";
import CreationButtons from "$lib/components/CreationButtons.svelte";
import MainLayout from "$lib/layouts/MainLayout.svelte";
import AcceptCancelButtons from "$lib/components/AcceptCancelButtons.svelte";
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
<!--        <div class="form-control w-full max-w-xs">-->
<!--            <label class="label" for="transaction_name">-->
<!--                <span class="label-text">What do you want to call this source?</span>-->
<!--            </label>-->
<!--            <input id="transaction_name" type="text" placeholder="Type here" class="input input-bordered input-sm w-full max-w-xs" bind:value={formData['name']} />-->
<!--        </div>-->

<!--        <div class="form-control w-full max-w-xs mt-2">-->
<!--            <label class="label" for="transaction_type">-->
<!--                <span class="label-text">Is it an Income or and Expense?</span>-->
<!--            </label>-->
<!--            <select id="transaction_type" class="select select-bordered select-sm" bind:value={formData['transaction_type']}>-->
<!--                <option disabled selected>Pick one</option>-->
<!--                <option>Income</option>-->
<!--                <option>Expense</option>-->
<!--            </select>-->
<!--        </div>-->

<!--        <div class="pt-2 flex justify-center">-->
<!--            <button class="btn btn-sm btn-primary" on:click={() => greet()}>Add</button>-->
<!--        </div>-->
    </div>

    <CreationButtons slot="footer"/>
</MainLayout>