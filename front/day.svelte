<script>
    export let day;
    $: weekend_class = "bg-danger text-light";
    $: selected_class = "bg-success bg-opacity-50 border-success";

    function toggleSelect(){
        day.isSelected = ! day.isSelected;
    }
</script>

{#if day }
<div class="flex-even text-center border {day.isSelected? selected_class :"border-light"} {day.isWeekend? weekend_class : ""} {day.isVacation? "bg-info":""}" 
on:click={toggleSelect} on:keydown={()=>{}}>
    { day.getTitle() }
</div>                        
{:else}
<div class="flex-even border border-white"></div>
{/if}
<style>
</style>
