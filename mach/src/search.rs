use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use slint::SharedString;

pub fn fuzzy_search<'a>(matcher: &SkimMatcherV2, user_query: &str, macro_candidates: &'a [SharedString], top_n: usize) -> Vec<&'a SharedString> {


    let mut scored: Vec<(&SharedString, i64)> = macro_candidates
        .iter()
        .filter_map(|cand| {
            //fuzzy match returns i64 score
            matcher.fuzzy_match(cand.as_str(), user_query).map(|score| (cand, score))
        })
        .collect();

    scored.sort_unstable_by(|a, b| b.1.cmp(&a.1)); //scores descending
    
    let result: Vec<&SharedString> = scored
        .into_iter()
        .take(top_n)
        .map(|(cand, _score)| cand)
        .collect();

    result
}
