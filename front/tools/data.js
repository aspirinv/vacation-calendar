export class vacation{
    from;
    to;
}

export class yearCalendar{
    year;
    ranges = [];
    
    contains(date){
        if(!date) return false;
        return this.ranges.some(r=>compareDates(date, r.from) >= 0 && compareDates(date, r.to) <= 0);
    }    
}

export class vacationClientFactory{
    createClient(){
        return new vacationLocalClient()
    }
}


export class vacationClient{
    async getVacations(year){
        (await (await fetch(`/api/vacation/${year}`)).json()).map(e=> new vacation())
    }
}

export class vacationLocalClient{
    async getVacations(year){
        const asDate = s => new Date(Date.parse(s));

        const dummy = new yearCalendar();
        dummy.year = year;
        dummy.ranges =  [{from:asDate("2023-03-20"), to: asDate('2023-03-24')}, {from: asDate("2023-05-02"), to: asDate("2023-05-10")}];
        return dummy;//JSON.parse(localStorage.getItem("vaca_" + year));
    }
    async setVacations(year, yearCalendar){
        return localStorage.setItem("vaca_" + year, JSON.stringify(yearCalendar));
    }

}

export const monthes = Array(12).fill(1).map((e,i)=>new Date(Date.parse("2023-"+(i+1))).toLocaleString(navigator.language,{month:'long'}));
export const compareDates = (a,b)=> a.getFullYear() !== b.getFullYear() 
    ? a.getFullYear() > b.getFullYear()?1:-1
        : a.getMonth() !== b.getMonth()? a.getMonth() > b.getMonth()?1:-1
            : a.getDate() !== b.getDate()?a.getDate() > b.getDate()?1:-1
            : 0;