export interface TimeSeriesData {
    date: Date;
    value: number;
}

export interface BodyOverview {
    today: {
        mass: number,
        fat: number
    }
    monthly: {
        mass: Array<TimeSeriesData>,
        fat: Array<TimeSeriesData>
    }
}