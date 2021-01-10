export interface TimeSeriesData {
    date: Date;
    value: number;
}

export interface BodyLog {
    date: Date;
    mass: number;
    fat: number;
}

export interface Today {
    mass: number;
    fat: number;
}

export interface BodyOverview {
    today: Today | undefined;
    past: Array<BodyLog>;
}