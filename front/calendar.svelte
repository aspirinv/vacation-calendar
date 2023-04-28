<script>
    import { datesBuilder } from './tools/datesBuilder.js'
    import { vacationClientFactory, monthes } from './tools/data.js'
    import { _0CircleFill } from 'svelte-bootstrap-icons';
    import Day from './day.svelte';

    let builder = null;   
    const client = new vacationClientFactory().createClient();
    let vacationCalendar;
    
    client.getVacations(2023).then(r=>{
        vacationCalendar = r;
        builder = new datesBuilder(2023, vacationCalendar);
    });

    $: weekend_class = "bg-danger text-light";
    $: selected_class = "bg-success bg-opaciity-50 border-success";

</script>

<div class="container-fluid">
    {#if  builder !== null}
    <div class="row user-select-none" >
        {#each Array(12) as _, i}

        <div class="mt-2 d-flex flex-column position-relative col-xl-2 col-lg-3 col-md-4 col-sm-6 col-xs-12">
            <div class="bg-light text-center text-primary">{ monthes[i] }</div>
            <div class="border border-light d-flex flex-row bg-primary text-light">        
                <div class="flex-even">Mo</div>
                <div class="flex-even">Di</div>
                <div class="flex-even">Mi</div>
                <div class="flex-even">Do</div>
                <div class="flex-even">Fr</div>
                <div class="flex-even">Sa</div>
                <div class="flex-even">So</div>

            </div>
            {#each builder.getWeeksOfMonth(i) as w }
                <div class="d-flex" >
                    {#each w.days as d, i}
                        <Day day={d}></Day>
                    {/each}
                </div>        
            {/each}
            </div>
        {/each}
    </div>
    {/if}
</div>
<style>

</style>
