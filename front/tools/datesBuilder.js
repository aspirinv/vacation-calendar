export class datesBuilder{
    
    constructor(year, vacationCalendar){
        this.year = year;
        this.vacationCalendar = vacationCalendar;
    }

    getWeeksOfMonth(month){
        console.log("getWeeksOfMonth called for ", month);
        let startDate = new Date(this.year, month, 1);
        let weeks = [];
        let w = new week();
        let firstDay = startDate.getDay();
        if(firstDay === 0){ //sunday
            firstDay = 7;
        }
        w.days = Array(firstDay - 1).fill(null);
        
        while(startDate.getMonth() === month){
            const d = startDate.getDay();
            const date = new day(new Date(startDate));
            date.isVacation = this.vacationCalendar.contains(date.getDate())
            w.days.push(date);
            if(d === 0){
                weeks.push(w);
                w = new week();
            }
            startDate.setDate(startDate.getDate() + 1);
        }

        if(w.days.length < 7){
            w.days.push(...Array(7 - w.days.length).fill(null));
        }
        weeks.push(w);
        return weeks;
    }

}

export class week{
    days = [];

}

export class day{
    
    _isSelected = false;
    constructor(date){
        this.date = date;
        this.isWeekend = date.getDay()%7 === 0 || date.getDay()%7 === 6;
    }

    getTitle(){        
        return this.date.getDate();
    }

    getDate(){
        return this.date;
    }

    get isSelected(){
        return this._isSelected;
    }

    set isSelected(val){
        this._isSelected = val;
    }
}
