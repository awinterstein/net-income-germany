//! Tax and social security configurations (e.g, the rates to apply on the income) per year.
//!
//! There are configurations available based on the German laws for the years 2024 and 2025.

// values for the social security (health and retirement) can be found on the website of the health ministry:
// https://www.bundesgesundheitsministerium.de/beitraege

/// Configuration for the state-operated health insurance used as part of the social security calculations.
#[derive(Debug)]
pub struct HealthInsuranceConfig {
    /// General premium value for the health insurance \[0,1\] (Beitragssatz)
    pub premium_general: f32,
    /// Reduced premium value \[0,1\] that applies for self-employed persons without sick pay (reduzierter Beitragssatz)
    pub premium_general_reduced: f32,
    /// Additional premium value \[0,1\] that is defined per insurance company (Zusatzbeitrag)
    pub premium_additional: f32,
    /// General premium value for the nursing insurance \[0,1\] (Beitragssatz Pflegeversicherung)
    pub premium_nursing: f32,
    /// Additional premium value \[0,1\], that is defined based on the amount of children that a person has (Zuschlag für Kinderlose)
    pub premium_nursing_additional: f32,
    /// Minimum monthly income that is used for the health insurance calculation, but only for self-employed persons (Mindestbeitrag)
    pub min_income: f32,
    /// Maximum monthly income that is used for the health insurance calculation (Beitragsbemessungsgrenze)
    pub max_income: f32,
}

/// Configuration for the state-operated retirement insurance used as part of the social security calculations.
#[derive(Debug)]
pub struct RetirementInsuranceConfig {
    /// Premium value for the retirement insurance (Beitragssatz)
    pub premium: f32,
    /// Maximum monthly income that is used for the retirement insurance calculation (Beitragsbemessungsgrenze)
    pub max_income: f32,
}

/// Configuration for the state-operated unemployment insurance used as part of the social security calculations.
#[derive(Debug)]
pub struct UnemploymentInsuranceConfig {
    /// Premium value for the unemployment insurance (Beitragssatz)
    pub premium: f32,
    /// Maximum monthly income that is used for the unemployment insurance calculation (Beitragsbemessungsgrenze)
    pub max_income: f32,
}

/// The income tax is calculated in multiple, progressive income ranges. This defines one range.
#[derive(Debug, Clone)]
pub struct TaxRange {
    /// The gross income from which this rax range applies.
    pub lower_limit: u32,
    /// The gross income up to which this range applies.
    pub upper_limit: u32,
    /// The lowest rate \[0,1\] within this tax range.
    pub rate_min: f32,
    /// The maximum rate \[0,1\] within this tax range.
    pub rate_max: f32,
}

/// Configuration for the additional solidarity tax that applies on large incomes.
#[derive(Debug)]
pub struct SolidaryAdditionConfig {
    /// The income tax value up to which the solidarity tax does not apply.
    pub exemption_level: u32,
    /// The tax rate \[0,1\] that is applied on the payed income tax.
    pub rate: f32,
    /// The limit for solidarity tax \[0,1\] that is applied on the income above the exemption level.
    pub max_percentage: f32,
}

/// Configuration for the income tax calculations.
#[derive(Debug)]
pub struct IncomeTaxConfig {
    /// All the progressive tax ranges of the income tax.
    pub tax_ranges: Vec<TaxRange>,

    /// Configuration for the additional solidarity tax that applies on large incomes.
    pub solidary_addition_config: SolidaryAdditionConfig,
}

/// Main configuration struct that contains all the needed tax and social security configurations.
#[derive(Debug)]
pub struct Config {
    pub health_insurance: HealthInsuranceConfig,
    pub retirement_insurance: RetirementInsuranceConfig,
    pub unemployment_insurance: UnemploymentInsuranceConfig,
    pub income_tax: IncomeTaxConfig,
}

impl Default for Config {
    /// Create configuration for the current year by default.
    fn default() -> Self {
        return create(2025).unwrap();
    }
}

/// Creates the configuration for the given year.
///
/// This function supports the years 2024 and 2025 and returns an error for every other year.
pub fn create(year: u32) -> Result<Config, &'static str> {
    match year {
        2025 => Ok(Config {
            retirement_insurance: RetirementInsuranceConfig {
                premium: 0.186,
                max_income: 8050.0,
            },
            health_insurance: HealthInsuranceConfig {
                premium_general: 0.146,
                premium_general_reduced: 0.14,
                premium_additional: 0.0245,
                premium_nursing: 0.036,
                premium_nursing_additional: 0.006,
                min_income: 1248.32,
                max_income: 5512.5,
            },
            unemployment_insurance: UnemploymentInsuranceConfig {
                premium: 0.026,
                max_income: 8050.0,
            },
            income_tax: IncomeTaxConfig {
                tax_ranges: vec![
                    TaxRange {
                        lower_limit: 0,
                        upper_limit: 12096,
                        rate_min: 0.00,
                        rate_max: 0.00,
                    },
                    TaxRange {
                        lower_limit: 12096,
                        upper_limit: 17444,
                        rate_min: 0.14,
                        rate_max: 0.2397,
                    },
                    TaxRange {
                        lower_limit: 17444,
                        upper_limit: 68480,
                        rate_min: 0.2397,
                        rate_max: 0.42,
                    },
                    TaxRange {
                        lower_limit: 68480,
                        upper_limit: 277825,
                        rate_min: 0.42,
                        rate_max: 0.42,
                    },
                    TaxRange {
                        lower_limit: 277825,
                        upper_limit: u32::MAX,
                        rate_min: 0.45,
                        rate_max: 0.45,
                    },
                ],
                solidary_addition_config: SolidaryAdditionConfig {
                    exemption_level: 19950,
                    rate: 0.055,
                    max_percentage: 0.119,
                },
            },
        }),
        2024 => Ok(Config {
            retirement_insurance: RetirementInsuranceConfig {
                premium: 0.186,
                max_income: 7550.0,
            },
            health_insurance: HealthInsuranceConfig {
                premium_general: 0.146,
                premium_general_reduced: 0.14,
                premium_additional: 0.012,
                premium_nursing: 0.034,
                premium_nursing_additional: 0.006,
                min_income: 1178.33,
                max_income: 5175.0,
            },
            unemployment_insurance: UnemploymentInsuranceConfig {
                premium: 0.026,
                max_income: 7550.0,
            },
            income_tax: IncomeTaxConfig {
                tax_ranges: vec![
                    TaxRange {
                        lower_limit: 0,
                        upper_limit: 11784,
                        rate_min: 0.00,
                        rate_max: 0.00,
                    },
                    TaxRange {
                        lower_limit: 11784,
                        upper_limit: 17005,
                        rate_min: 0.14,
                        rate_max: 0.2397,
                    },
                    TaxRange {
                        lower_limit: 17005,
                        upper_limit: 66760,
                        rate_min: 0.2397,
                        rate_max: 0.42,
                    },
                    TaxRange {
                        lower_limit: 66760,
                        upper_limit: 277825,
                        rate_min: 0.42,
                        rate_max: 0.42,
                    },
                    TaxRange {
                        lower_limit: 277825,
                        upper_limit: u32::MAX,
                        rate_min: 0.45,
                        rate_max: 0.45,
                    },
                ],
                solidary_addition_config: SolidaryAdditionConfig {
                    exemption_level: 18130,
                    rate: 0.055,
                    max_percentage: 0.119,
                },
            },
        }),
        _ => Err("No configuration available for given year."),
    }
}
