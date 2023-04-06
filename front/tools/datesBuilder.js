export class datesBuilder{
    
    constructor(year){
        this.year = year;
    }

    getWeeksOfMonth(month){
        let startDate = new Date(this.year, month, 1);
        let weeks = [];
        let w = new week();
        let firstDay = startDate.getDay();
        if(firstDay === 0){ //sunday
            firstDay = 7;
        }
        w.days = Array(firstDay - 1).fill(new defaultDay());
        
        while(startDate.getMonth() === month){
            const d = startDate.getDay();
            w.days.push(new day(new Date(startDate)));
            if(d === 0){
                weeks.push(w);
                w = new week();
            }
            startDate.setDate(startDate.getDate() + 1);
        }

        if(w.days.length < 7){
            w.days.push(...Array(7 - w.days.length).fill(new defaultDay()));
        }
        weeks.push(w);
        return weeks;
    }

}

export class week{
    days = [];

}

export class day{
    
    constructor(date){
        this.date = date;
        this.isWeekend = date.getDay()%7 === 0 || date.getDay()%7 === 6;
    }

    getTitle(){        
        return this.date.getDate();
    }
}

export class defaultDay{
    isWeekend = false;
    getTitle = () => "";
}
