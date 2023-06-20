use relative_path::RelativePath;
use url::Url;

pub fn remap_icon(icon_old: &str, relative_scrapbook_path: &RelativePath) -> Option<String> {
    match Url::parse(icon_old) {
        Ok(url) => match url.scheme() {
            "moz-icon" => None,
            "resource" => {
                if matches!(url.domain(), Some("scrapbook")) {
                    let root_to_scrapbook: String = relative_scrapbook_path
                        .components()
                        .map(|component| format!("/{}", urlencoding::encode(component.as_str())))
                        .collect();
                    let scrapbook_to_icon: String = url
                        .path_segments()?
                        .map(|segment| format!("/{segment}"))
                        .collect();

                    Some(format!(
                        "/files/scrapbook{root_to_scrapbook}{scrapbook_to_icon}"
                    ))
                } else {
                    None
                }
            }
            _ => Some(url.to_string()),
        },
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::path_convert::ToRelativePath;

    use super::*;

    #[test]
    fn moz_icon() {
        assert_eq!(
            remap_icon(
                "moz-icon://fulltext.pdf?size=16",
                &RelativePath::new("scrap"),
            ),
            None
        )
    }

    #[test]
    fn resource() {
        assert_eq!(
            remap_icon(
                "resource://scrapbook/data/20061213170100/favicon.ico",
                &RelativePath::new("scrap"),
            )
            .as_deref(),
            Some("/files/scrapbook/scrap/data/20061213170100/favicon.ico")
        )
    }

    #[test]
    fn resource_nopath() {
        let scrapbook_path = Path::new("/home/user/scrapbook");

        assert_eq!(
            remap_icon(
                "resource://scrapbook/data/20061213170100/favicon.ico",
                &scrapbook_path.to_relative_path(scrapbook_path).unwrap(),
            )
            .as_deref(),
            Some("/files/scrapbook/data/20061213170100/favicon.ico")
        )
    }

    #[test]
    fn http() {
        assert_eq!(
            remap_icon("https://foo.com/favicon.ico", &RelativePath::new("scrap"),).as_deref(),
            Some("https://foo.com/favicon.ico")
        )
    }
}
