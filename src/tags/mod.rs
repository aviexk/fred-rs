
// MIT License
// 
// Copyright (c) 2020 Matthew Sabo
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

/// Get the series matching tags
/// 
/// [https://research.stlouisfed.org/docs/api/fred/tags_series.html](https://research.stlouisfed.org/docs/api/fred/tags_series.html)
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::tags::series::{Builder, SortOrder, OrderBy};
/// use fred_rs::series::Response;
/// 
/// let mut c = match FredClient::new() {
///     Ok(c) => c,
///     Err(msg) => {
///         println!("{}", msg);
///         assert_eq!(2, 1);
///         return
///     },
/// };
/// 
/// let mut builder = Builder::new();
/// builder
///     .tag_name("usa")
///     .limit(5)
///     .sort_order(SortOrder::Descending)
///     .order_by(OrderBy::Popularity);
/// 
/// let resp: Response = match c.tags_series(builder) {
///     Ok(resp) => resp,
///     Err(msg) => {
///         println!("{}", msg);
///         assert_eq!(2, 1);
///         return
///     },
/// };
/// 
/// for item in resp.seriess {
///     println!(
///         "{}: {}",
///         item.id,
///         item.title
///     );
/// }
/// ```
pub mod series;

// -----------------------------------------------------------------------------

use serde::Deserialize;

#[derive(Deserialize)]
/// Response data structure for the fred/tags endpoint
/// 
/// [https://research.stlouisfed.org/docs/api/fred/tags.html] (https://research.stlouisfed.org/docs/api/fred/tags.html)
pub struct Response {
    /// The Real Time start date for the request
    pub realtime_start: String,
    /// The Real Time end data for the request
    pub realtime_end: String,
    /// How the results are ordered
    pub order_by: String,
    // Results are listed in ascending or descending
    pub sort_order: String,
    /// Number of results returned
    pub count: usize,
    /// ???
    pub offset: usize,
    /// Maximum number of results to return
    pub limit: usize,
    /// Series returned by the search
    pub tags: Vec<Tag>,
}

#[derive(Deserialize)]
/// Data structure containing infomation about a particular tag
/// 
/// [https://research.stlouisfed.org/docs/api/fred/tags.html](https://research.stlouisfed.org/docs/api/fred/tags.html)
pub struct Tag {
    /// The tag name
    pub name: String,
    /// The group ID string
    pub group_id: String,
    /// Additonal information about the tag (e.g. authors or sources)
    pub notes: Option<String>,
    /// Date and time the tag was created
    pub created: String,
    /// Popularity score
    pub popularity: isize,
    /// Number of series with the tag
    pub series_count: usize,
}

/// A tag group id to filter tags by type.
/// 
/// https://research.stlouisfed.org/docs/api/fred/tags.html#tag_group_id](https://research.stlouisfed.org/docs/api/fred/tags.html#tag_group_id)
pub enum TagGroupId {
    Frequency,
    General,
    Geography,
    GeographyType,
    Release,
    SeasonalAdjustment,
    Source,
    CitationAndCopyright,
}

/// Determines the order of search results
/// 
/// [https://research.stlouisfed.org/docs/api/fred/tags.html#order_by](https://research.stlouisfed.org/docs/api/fred/tags.html#order_by)
pub enum OrderBy {
    /// Default
    SeriesCount,
    Popularity,
    Created,
    Name,
    GroupId,
}

/// Sort order options for the fred/series/tags endpoint
/// 
/// [https://research.stlouisfed.org/docs/api/fred/tags.html#sort_order](https://research.stlouisfed.org/docs/api/fred/tags.html#sort_order)
pub enum SortOrder {
    /// Dates returned in ascending order (default)
    Ascending,    
    /// Dates returned in descending order
    Descending,   
}

pub struct Builder {
    option_string: String,
    tag_names: String,
}

impl Builder {

    /// Initializes a new tags::Builder that can be used to add arguments to an API request
    /// 
    /// The builder does not do validity checking of the arguments nor does it check for duplicates.
    /// 
    /// ```
    /// use fred_rs::tags::Builder;
    /// // Create a new builder
    /// let mut builder = Builder::new();
    /// // add arguments to the builder
    /// builder
    ///     .realtime_start("1900-01-01")
    ///     .realtime_end("2000-01-01");
    /// ```
    pub fn new() -> Builder {
        Builder {
            option_string: String::new(),
            tag_names: String::new(),
        }
    }

    /// Returns the current arguments as a URL formatted string
    pub fn options(mut self) -> String {
        if self.tag_names.len() > 0 {
            self.option_string += format!("&tag_names={}", self.tag_names).as_str()
        }
        self.option_string
    }

    /// Adds a realtime_start argument to the builder
    /// 
    /// # Arguments
    /// * `start_date` - date formatted as YYYY-MM-DD
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/tags.html#realtime_start](https://research.stlouisfed.org/docs/api/fred/tags.html#realtime_start)
    pub fn realtime_start(&mut self, start_date: &str) -> &mut Builder {
        self.option_string += format!("&realtime_start={}", start_date).as_str();
        self
    }

    /// Adds a realtime_end argument to the builder
    /// 
    /// # Arguments
    /// * `end_date` - date formatted as YYYY-MM-DD
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/tags.html#realtime_end](https://research.stlouisfed.org/docs/api/fred/tags.html#realtime_end)
    pub fn realtime_end(&mut self, end_date: &str) -> &mut Builder {
        self.option_string += format!("&realtime_end={}", end_date).as_str();
        self
    }

    /// Adds a tag name to include in the search
    /// 
    /// Results must match all included tag names.
    /// 
    /// # Arguments
    /// * `tag` - tag name to add
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/tags.html#tag_names](https://research.stlouisfed.org/docs/api/fred/tags.html#tag_names)
    pub fn tag_name(&mut self, tag: &str) -> &mut Builder {
        if self.tag_names.len() != 0 {
            self.tag_names.push(';');
        } 
        self.tag_names += tag;
        self
    }

    /// Adds a group id filter to the results
    /// 
    /// # Arguments
    /// * `id` - type by which to filter results
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/tags.html#tag_group_id](https://research.stlouisfed.org/docs/api/fred/tags.html#tag_group_id)
    pub fn tag_group_id(&mut self, id: TagGroupId) -> &mut Builder {
        match id {
            TagGroupId::Frequency => {
                self.option_string += "&tag_group_id=freq";
            },
            TagGroupId::General => {
                self.option_string += "&tag_group_id=gen";
            },
            TagGroupId::Geography => {
                self.option_string += "&tag_group_id=geo";
            },
            TagGroupId::GeographyType => {
                self.option_string += "&tag_group_id=geot";
            },
            TagGroupId::Release => {
                self.option_string += "&tag_group_id=rls";
            },
            TagGroupId::SeasonalAdjustment => {
                self.option_string += "&tag_group_id=seas";
            },
            TagGroupId::Source => {
                self.option_string += "&tag_group_id=src";
            },
            TagGroupId::CitationAndCopyright => {
                self.option_string += "&tag_group_id=cc";
            },
        };
        self
    }

    /// Search string to find matching tags with
    /// 
    /// # Arguments
    /// * `text` - text to search against
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/tags.html#search_text](https://research.stlouisfed.org/docs/api/fred/tags.html#search_text)
    pub fn search_text(&mut self, text: &str) {
        self.option_string += format!("&search_text={}", text).as_str();
    }

    /// Adds a limit argument to the builder
    /// 
    /// The limit argument specifies a maximum number of observations to return.
    /// 
    /// # Arguments
    /// * `num_results` - Maximum number of results to return
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/tags.html#limit](https://research.stlouisfed.org/docs/api/fred/tags.html#limit)
    pub fn limit(&mut self, num_results: usize) -> &mut Builder {
        let num_results = if num_results > 1000 { // max value is 1000
            1000
        } else {
            num_results
        };
        self.option_string += format!("&limit={}", num_results).as_str();
        self
    }

    /// Adds an offset argument to the builder
    /// 
    /// Adding an offset shifts the starting result number.  For example, if limit is 5 and offset is 0 then results 1-5 will be returned, but if offset was 5 then results 6-10 would be returned.
    /// 
    /// # Arguments
    /// * `ofs` - the offset amount
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/tags.html#offset](https://research.stlouisfed.org/docs/api/fred/tags.html#offset)
    pub fn offset(&mut self, ofs: usize) -> &mut Builder {
        self.option_string += format!("&offset={}", ofs).as_str();
        self
    }

    /// Adds the search_type argument to the request
    /// 
    /// # Arguments
    /// * `order` - result ranking system
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/tags.html#order_by](https://research.stlouisfed.org/docs/api/fred/tags.html#order_by)
    pub fn order_by(&mut self, order: OrderBy) -> &mut Builder {
        match order {
            OrderBy::SeriesCount => {
                self.option_string += "&order_by=series_count";
            },
            OrderBy::Popularity => {
                self.option_string += "&order_by=popularity";
            },
            OrderBy::Created => {
                self.option_string += "&order_by=created";
            },
            OrderBy::Name => {
                self.option_string += "&order_by=name";
            },
            OrderBy::GroupId => {
                self.option_string += "&order_by=group_id";
            },
        };
        self
    }

    /// Change the sort order of the data
    /// 
    /// # Arguments
    /// * `order` - Data sort order enum
    /// 
    /// [https://research.stlouisfed.org/docs/api/fred/tags.html#sort_order](https://research.stlouisfed.org/docs/api/fred/tags.html#sort_order)
    pub fn sort_order(&mut self, order: SortOrder) -> &mut Builder {
        match order {
            SortOrder::Descending => {
                self.option_string += format!("&sort_order=desc").as_str()
            },
            _ => () // ASC is the default so do nothing
        }
        self
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::FredClient;

    #[test]
    fn tags_with_options() {
        let mut c = match FredClient::new() {
            Ok(c) => c,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        let mut builder = Builder::new();
        builder
            .sort_order(SortOrder::Descending)
            .order_by(OrderBy::Popularity)
            .limit(5);

        let resp: Response = match c.tags(Some(builder)) {
            Ok(resp) => resp,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        for item in resp.tags {
            println!(
                "{}: {}",
                item.name,
                item.created,
            );
        }
    } 
}