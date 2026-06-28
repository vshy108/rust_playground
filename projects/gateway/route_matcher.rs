use crate::Route;

// Selects the best matching route for `path` using prefix matching with specificity scoring.
//
// Matching rules:
//   - Split both sides by '/' (dropping empty strings from the leading slash).
//   - Path must have at least as many segments as the prefix.
//   - A prefix segment starting with ':' is a parameter — it matches any path segment.
//   - All other prefix segments must match the path segment exactly.
//
// Score: (segment_count, static_count).
//   - More segments = more specific, so a deeper route wins a shallower one.
//   - At equal depth, more static segments beat more param segments —
//     "/users/admin" wins over "/users/:id" for path "/users/admin".
//     This mirrors specificity-based routers such as axum/matchit and Fastify.
//   - The root prefix "/" has 0 segments and 0 static segments, so it is always the
//     lowest-scoring match — a natural catch-all fallback.
pub fn match_route<'routes>(path: &str, routes: &'routes [Route]) -> Option<&'routes Route> {
    // 'routes is a named lifetime — the returned &Route is a reference into the `routes` slice,
    // not into `path`. Naming it here tells the compiler: "the returned reference lives as long
    // as `routes` does", so it can verify that at every call site.

    // Tracks the best match found so far. None = no match yet.
    // When a candidate is found, stores Some((score, &route)) where score = (depth, static_count).
    let mut best: Option<((usize, usize), &Route)> = None;

    for route in routes {
        // "/users/admin".split('/') produces ["", "users", "admin"] because the leading '/'
        // creates an empty string before the first segment. filter removes that empty string,
        // leaving ["users", "admin"]. Same treatment for the incoming path.
        let prefix_segs: Vec<&str> = route.prefix.split('/').filter(|s| !s.is_empty()).collect();
        let path_segs: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        // A prefix with 2 segments can't match a path with only 1 segment — skip early.
        if path_segs.len() < prefix_segs.len() {
            continue;
        }

        // zip pairs up segments side by side: ("users","users"), (":id","42"), …
        // all() returns true only if every pair satisfies the closure:
        //   p.starts_with(':') — prefix segment is a param like :id, accepts any value
        //   p == s             — static segment must match exactly
        // If any pair fails, matched = false and we skip this route.
        let matched = prefix_segs
            .iter()
            .zip(path_segs.iter())
            .all(|(p, s)| p.starts_with(':') || p == s);
        if !matched {
            continue;
        }

        // Count how many prefix segments are static (not params).
        // Score is a tuple (depth, static_count):
        //   "/users/admin" → (2, 2)   "/users/:id" → (2, 1)   "/users" → (1, 1)   "/" → (0, 0)
        let static_count = prefix_segs.iter().filter(|s| !s.starts_with(':')).count();
        let score = (prefix_segs.len(), static_count);

        // Rust tuple comparison is lexicographic: (2,2) > (2,1) > (1,1) > (0,0).
        // So deeper routes beat shallower ones; at equal depth, more-static beats more-param.
        //
        // If best already has a score >= this candidate — do nothing (keep the existing best).
        // The >= (not >) means: on a tie, the first configured route in the slice wins.
        // Otherwise (_) — this candidate is better, replace best.
        match best {
            Some((best_score, _)) if best_score >= score => {}
            _ => best = Some((score, route)),
        }
    }

    // Discard the score tuple; return just the &Route.
    // If best is still None (no route matched at all), returns None.
    best.map(|(_, route)| route)
}
