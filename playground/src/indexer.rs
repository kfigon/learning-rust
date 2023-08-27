use std::collections::HashMap;

static DATA: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean varius venenatis dignissim. Integer aliquet bibendum tellus, nec maximus ipsum faucibus non. Nunc est felis, auctor in tincidunt at, interdum vitae felis. Donec egestas tellus vitae mauris molestie, sit amet molestie dui ornare. Pellentesque at nulla tristique, tincidunt tellus sit amet, placerat turpis. Ut pulvinar tincidunt eleifend. Vivamus quis posuere mi. Phasellus laoreet sem eu vehicula porta. Fusce vehicula eros id turpis vestibulum, eget vulputate neque egestas. Proin sodales molestie sem, et cursus leo interdum eget. Suspendisse potenti.

In mollis metus mauris, quis convallis urna iaculis at. Maecenas elementum non mauris nec fermentum. Suspendisse vulputate id nisi eu scelerisque. Nulla porta quam eget vehicula condimentum. Mauris sagittis, purus eu vestibulum vestibulum, sapien ipsum ullamcorper nulla, nec rutrum massa sem at quam. Sed mollis nibh eu nulla fermentum, id sagittis eros imperdiet. Mauris vel efficitur felis. Etiam porta libero eu auctor aliquet. Aenean ornare volutpat mi a venenatis. Aenean at laoreet libero. In hac habitasse platea dictumst. Curabitur id elementum tellus, eget dictum ligula. Donec bibendum feugiat porttitor. Quisque eget ex lectus. Etiam fringilla a erat in lobortis.

Etiam at tortor vitae nibh ultrices aliquam. Proin a feugiat metus. Integer sodales aliquam odio, vel mattis metus volutpat ac. Morbi eget magna est. Interdum et malesuada fames ac ante ipsum primis in faucibus. Ut ac urna faucibus, vestibulum mauris at, luctus lacus. Nam accumsan quam lectus, ut convallis augue gravida a. Maecenas interdum elementum ipsum, vitae consectetur lectus tempor in. Etiam imperdiet nisi ut est finibus commodo. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus venenatis risus feugiat metus pellentesque, at mattis erat sollicitudin. Aliquam iaculis hendrerit ex sed fermentum. Sed metus sem, mollis sed eros sit amet, feugiat feugiat diam. Fusce arcu magna, pellentesque non tellus eget, gravida sodales quam. Aliquam sit amet tortor in tellus congue ullamcorper ut et massa.

Phasellus diam mi, cursus eu est vel, euismod bibendum enim. Quisque suscipit cursus ipsum, sed faucibus nisi gravida quis. Pellentesque blandit bibendum viverra. Donec nec libero nec nunc dignissim dignissim. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Integer sollicitudin orci iaculis, varius nisl eu, molestie mauris. Duis pellentesque tristique augue sit amet varius. Duis at maximus erat, a ornare augue. Sed egestas dignissim ante, et pellentesque dui porttitor ac. Sed in est tempus, vulputate nulla non, ultrices nisl. Suspendisse iaculis massa vel rhoncus vulputate. Quisque convallis felis in felis feugiat, vel convallis lectus pharetra. Curabitur ac aliquet eros. Donec luctus congue venenatis. Quisque ut urna ut turpis dapibus tincidunt.

Morbi hendrerit consequat hendrerit. Vivamus pharetra posuere facilisis. Etiam dui sapien, sagittis vitae fringilla vitae, varius et nisi. Integer mi dui, finibus a tincidunt nec, eleifend eget neque. Cras nec tellus blandit, dictum augue et, interdum dui. Morbi tempor auctor augue at dictum. Suspendisse id cursus ligula. Duis vehicula volutpat diam, congue sodales justo feugiat sed. Curabitur posuere, mi sed elementum rhoncus, magna nisi auctor nunc, eu fermentum lectus ex at purus. Fusce dictum molestie dui, ut fringilla ipsum maximus ullamcorper. Nam nibh arcu, placerat faucibus imperdiet et, sodales mattis ex. Integer at ligula lacinia, lacinia sem sit amet, consequat dui. Vestibulum in ipsum sit amet justo sollicitudin facilisis. In ultricies malesuada sapien vel faucibus. Nam pulvinar arcu et tortor convallis lobortis. Fusce sagittis, nulla nec rhoncus pellentesque, augue eros mattis mi, at euismod diam neque in sapien.";


fn index(text: &str, limit: usize) -> Vec<(String, usize)>{
    let mut map: HashMap<String, usize> = HashMap::new();

    text.split_whitespace()
        .map(&str::to_lowercase)
        .map(|word| word.replace(|c: char| !c.is_alphanumeric(), ""))
        .for_each(|w| *map.entry(w).or_default() += 1);

    let mut sorted: Vec<(&String, &usize)> = map.iter().collect();

    // sort by size (and name if equal)
    sorted.sort_by(|a, b| {
        match b.1.cmp(a.1) {
            std::cmp::Ordering::Equal => b.0.cmp(a.0),
            r => r
        }
    });

    match sorted.windows(limit).next() {
        None => vec![],
        Some(v) => v.iter().map(|entry| (entry.0.to_string(), *entry.1)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{index, DATA};

    #[test]
    fn lorem_test() {
        let res = index(DATA, 3);
        
        assert_eq!(res, vec![
            ("at".to_string(), 13),
            ("in".to_string(), 12),
            ("sit".to_string(), 9),
        ]);
    }
}