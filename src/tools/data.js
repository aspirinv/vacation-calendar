export class vacation{

}

export class vacationClient{
    async getVacations(year){
        (await (await fetch(`/api/vacation/${year}`)).json()).map(e=> new vacation())
    }
}