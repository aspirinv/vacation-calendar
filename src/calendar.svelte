<script>
    import { datesBuilder } from './tools/datesBuilder.js'

    const builder = new datesBuilder(2023);    

    $: weekend_class = "bg-danger text-light";
</script>

<div class="row">
    {#each Array(12) as _, i}

    <div class="d-flex flex-column position-relative col-md-3 col-sm-4 col-xs-12">
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
                    <div class="flex-even text-center border border-light {d.isWeekend? weekend_class : ""}" >{ d.getTitle() }</div>
                {/each}
            </div>        
        {/each}
        </div>
    {/each}
</div>

<style>
    .flex-even{
        flex: 1;
    }
</style>
