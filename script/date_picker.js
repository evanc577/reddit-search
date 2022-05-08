import flatpickr from "flatpickr";

const config = {
    enableTime: true,
    time_24hr: true,
    defaultHour: 0,
};

flatpickr("#time_start", config);
flatpickr("#time_end", config);
