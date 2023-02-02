#[derive(Debug, PartialEq, Eq)]
struct StandardProductIdentifier {
    numeric_code: i64,
    alphabetic_code: String,
    name: String,
}

impl StandardProductIdentifier {
    pub fn new(numeric_code: i64, alphabetic_code: &str, name: &str) -> Self {
        Self {
            numeric_code,
            alphabetic_code: alphabetic_code.to_owned(),
            name: name.to_owned(),
        }
    }
}

struct StandardProductIdentifierRepo {}

impl StandardProductIdentifierRepo {
    pub async fn get_by_numeric_code(code: i64) -> Result<StandardProductIdentifier, sqlx::Error> {
        todo!()
    }

    pub async fn get_by_alphabetic_code(
        alpha_code: &str,
    ) -> Result<StandardProductIdentifier, sqlx::Error> {
        todo!()
    }
    pub async fn save(spi: &StandardProductIdentifier) -> Result<(), sqlx::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{StandardProductIdentifier, StandardProductIdentifierRepo};

    #[actix_web::test]
    async fn can_save_and_get_spi() {
        let spi = StandardProductIdentifier::new(562, "UNBTTR", "Unsalted Butter");

        StandardProductIdentifierRepo::save(&spi).await;

        let returned_spi_by_code = StandardProductIdentifierRepo::get_by_numeric_code(562)
            .await
            .unwrap();
        let returned_spi_by_alpha_code =
            StandardProductIdentifierRepo::get_by_alphabetic_code("UNBTTR")
                .await
                .unwrap();

        assert_eq!(spi, returned_spi_by_code);
        assert_eq!(spi, returned_spi_by_alpha_code);
    }
}
