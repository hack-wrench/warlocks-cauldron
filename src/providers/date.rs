use super::dependencies::*;
pub use chrono::{prelude::*, Duration};
use chrono_tz::Tz;


pub enum TimestampType {
    POSIX(i64),
    RFC3339(String),
}

/// Struct for generating data related to the date and time
pub struct Datetime(pub Locale);

impl Datetime {
    /// Private. Return global parsed data from own locale
    fn data(&self) -> &ParsedData { self.0.get_data() }

    /// Private
    fn get_days_from_month(year: i32, month: u32) -> u32 {
        NaiveDate::from_ymd_opt(
            match month {
                12 => year + 1,
                _ => year,
            },
            match month {
                12 => 1,
                _ => month + 1,
            },
            1,
        ).unwrap()
        .signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).unwrap())
        .num_days() as u32
    }

    /// Bulk create datetime structs
    /// 
    /// This method creates list of datetime objects from
    /// ``date_start`` to ``date_end``
    ///
    /// return example: vec!\[DateTime, \]
    ///
    /// # Arguments
    /// * `date_start` - Begin of the range
    /// * `date_end` - End of the range
    /// * `delta` - Range delimetr
    pub fn bulk_create_datetimes<Tz: TimeZone>(date_start: DateTime<Tz>, date_end: DateTime<Tz>, delta: Duration) -> Vec<DateTime<Tz>>{
        if date_start > date_end {
            panic!("date_end must be greater than date_start!")
        }

        if delta.is_zero() {
            panic!("delta must not be zero!")
        }

        let mut v = vec![];
        let mut last_date = date_start;

        while last_date < date_end {
            last_date = last_date.checked_add_signed(delta).expect("Cant calculate delta!");
            if last_date < date_end {
                v.push(last_date.clone())
            }
        }

        v
    }

    /// Get a random periodicity string.
    ///
    /// return example: Periodicity
    pub fn periodicity(&self) -> &str {
        get_random_element(self.data().datetime.periodicity.iter())
    }
    
    /// Get a random century
    ///
    /// return example: XIII
    pub fn century() -> &'static str {
        get_random_element(ROMAN_NUMS.iter())
    }

    /// Generate a random year
    ///
    /// return example: 2013
    ///
    /// # Arguments
    /// * `minimum` - Minimum value
    /// * `maximum` - Maximum value
    pub fn year(minimum: i32, maximum: i32) -> i32 {
        randint(minimum, maximum)
    }

    /// Get a random month
    ///
    /// return example: Month name or abbr
    ///
    /// # Arguments
    /// * `abbr` - Abbreviated month name
    pub fn month(&self, abbr: bool) -> &str {
        get_random_element(match abbr {
            true => self.data().datetime.month.abbr.iter(),
            false => self.data().datetime.month.name.iter(),
        })
    }

    /// Get a random day of week
    /// 
    /// return example: Day name or abbr
    ///
    /// # Arguments
    /// * `abbr` - Abbreviated month name
    pub fn day_of_week(&self, abbr: bool) -> &str {
        get_random_element(match abbr {
            true => self.data().datetime.day.abbr.iter(),
            false => self.data().datetime.day.name.iter(),
        })
    }

    /// Get week number with year
    /// 
    /// return example: 2013-W13
    ///
    /// # Arguments
    /// * `start` - From start
    /// * `end` - To end
    pub fn week_data(&self, start: i32, end: i32) -> String {
        format!("{}-W{}", Self::year(start, end), randint(1, 52))
    }

    /// Generate random naive date
    /// 
    /// return example: NaiveDate
    ///
    /// # Arguments
    /// * `start` - Minimum value of year 
    /// * `end` - Maximum value of year
    pub fn date(start: i32, end: i32) -> NaiveDate {
        let year = randint(start, end);
        let month = randint(1, 12);
        let day = randint(1, Self::get_days_from_month(year, month));
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    /// Generate random date as string
    /// 
    /// return example: Formatted date
    ///
    /// # Arguments
    /// * `fmt` - The format of date, if None then use standard accepted in the current locale
    /// * `start` - Minimum value of year 
    /// * `end` - Maximum value of year
    pub fn formatted_date(&self, fmt: Option<&str>, start: i32, end: i32) -> String {
        let fmt = match fmt {
            Some(f) => f,
            None => &self.data().datetime.formats.date,
        };

        Self::date(start, end).format(fmt).to_string()
    }

    /// Generate a random naive time
    ///
    /// return example: NaiveTime
    pub fn time() -> NaiveTime {
        NaiveTime::from_hms_micro_opt(randint(0, 23), randint(0, 59), randint(0, 59), randint(0, 999999))
            .unwrap()
    }

    /// Generate string formatted time
    ///
    /// return example: formatted time
    /// 
    /// # Arguments
    /// * `fmt` - The format of time, if None then use standard accepted in the current locale
    pub fn formatted_time(&self, fmt: Option<&str>) -> String {
        let fmt = match fmt {
            Some(f) => f,
            None => &self.data().datetime.formats.time,
        };

        Self::time().format(fmt).to_string()
    }

    /// Generate a random day of month, from 1 to 31
    ///
    /// return example: 13
    pub fn day_of_month() -> i32 {
        randint(1, 31)
    }

    /// Get a random timezone
    ///
    /// return example: Antarctica/Troll
    /// 
    /// # Arguments
    /// * `region` - Timezone region
    pub fn timezone(region: Option<TimezoneRegion>) -> &'static str {
        let region_name = validate_enum(region, None);
        get_random_element(TIMEZONES.iter().filter(|tz| tz.starts_with(region_name)))
    }

    /// Get a random GMT offset value
    ///
    /// return example: UTC +13:00
    pub fn gmt_offset() -> &'static str {
        get_random_element(GMT_OFFSETS.iter())
    }

    /// Generate random datetime
    ///
    /// return example: DateTime<Utc>
    /// 
    /// # Arguments
    /// * `start` - Minimum value of year
    /// * `end` - Maximum value of year
    pub fn datetime(start: i32, end: i32) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(
            NaiveDateTime::new(Self::date(start, end), Self::time()), Utc,
        )
    }

    /// Generate datetime string in human readable format
    /// 
    /// return example: formatted datetime
    /// 
    /// # Arguments
    /// * `fmt` - The format of datetime, if None then use standard accepted in the current locale
    /// * `start` - Minimum value of year
    /// * `end` - Maximum value of year
    pub fn formatted_datetime(&self, fmt: Option<&str>, start: i32, end: i32) -> String {
        let full_format = {
            let data = self.data();
            format!("{} {}", &data.datetime.formats.date, &data.datetime.formats.time)
        };

        let fmt = match fmt {
            Some(f) => f,
            None => &full_format,
        };

        Self::datetime(start, end).format(fmt).to_string()
    }

    /// Generate random timestamp
    /// 
    /// return example: TimestampType::POSIX(133333333333)
    ///
    /// # Arguments
    /// * `posix` - Use POSIX time
    /// * `start` - Minimum value of year
    /// * `end` - Maximum value of year
    pub fn timestamp(posix: bool, start: i32, end: i32) -> TimestampType {
        match posix {
            true => TimestampType::POSIX(Self::datetime(start, end).timestamp()),
            false => TimestampType::RFC3339(Self::datetime(start, end).to_rfc3339()),
        }
    }
}
