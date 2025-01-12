# net-income-germany

Calculates the net income from a given yearly gross income regarding German social security and
income tax laws.

It can handle the calculations of the social security taxes for employed and for self-employed
income and can also take tax splitting for married couples into account.

The following taxes and social security fees are considered for the calculation of the net
income:
- health insurance (Gesetzliche Krankenversicherung)
- nursing care insurance (Pflegeversicherung)
- unemplyoment insurance (Arbeitslosenversicherung)
- income tax (Einkommenssteuer)
- solidarity surcharge (Solidaritätszuschlag)

## Example
```rust
// set the necessary input data values
let tax_data = net_income_germany::TaxData {
    gross_income: 80000, // the gross income of one year
    expenses: 5300, // the tax-deductible expenses of one year
    fixed_retirement: Some(800), // an optional fixed monthly retirement rate (otherwise percentage applies)
    self_employed: false, // whether social security taxes should be calculated for a self-employed person
    married: false, // whether tax splitting due to marriage should apply
};

// create the default configuration for a specific year (2024 and 2025 are supported)
let config = net_income_germany::config::create(2025)?;

// do the tax calculation based on the input data values
let tax_result = net_income_germany::calculate(&config, &tax_data)?;

// access the results (in this example just the resulting net income)
println!("Net income: {}", tax_result.net_income);

```

## Configuration Value Adaptions

The configuration applies the additional fee to the nursing care insurance that is obligatory
for childless people over 23 years old. For the additional health insurance fee, it applies the
fee of Techniker Krankenkasse. In case that you want to change any of those to other values, you
can do it as follows:

```rust
let mut config = net_income_germany::config::create(2025)?;
config.health_insurance.premium_additional = 0.0025; // change the additional health insurance fee [0,1]
config.health_insurance.premium_nursing_additional = 0.002; // change the additional nursing insurance fee [0,1]

```

License: MPL-2.0
