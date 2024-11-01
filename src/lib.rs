#![allow(rustdoc::private_intra_doc_links)]
#![deny(
    // Documentation
	// TODO: rustdoc::broken_intra_doc_links,
	// TODO: rustdoc::missing_crate_level_docs,
	// TODO: missing_docs,
	// TODO: clippy::missing_docs_in_private_items,

    // Other
	deprecated_in_future,
	exported_private_dependencies,
	future_incompatible,
	missing_copy_implementations,
	missing_debug_implementations,
	rust_2018_compatibility,
	rust_2018_idioms,
	trivial_casts,
	trivial_numeric_casts,
	unsafe_code,
	unstable_features,
	unused_import_braces,
	unused_qualifications,

	// clippy attributes
	clippy::missing_const_for_fn,
	clippy::redundant_pub_crate,
	clippy::use_self
)]
#![cfg_attr(docsrs, feature(doc_cfg), feature(doc_alias))]

use serde::{Deserialize, Serialize};
#[cfg(feature = "validate")]
use serde_valid::Validate;
use std::fmt;

/// Resume Schema
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Resume {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub basics: Option<Basics>,
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub work: Vec<Work>,
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub volunteer: Vec<Volunteer>,
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub education: Vec<Education>,
	/// Specify any awards you have received throughout your professional career
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub awards: Vec<Award>,
	/// Specify any certificates you have received throughout your professional career
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub certificates: Vec<Certificate>,
	/// Specify your publications through your career
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub publications: Vec<Publication>,
	/// List out your professional skill-set
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub skills: Vec<Skill>,
	/// List any other languages you speak
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub languages: Vec<Language>,
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub interests: Vec<Interest>,
	/// List references you have received
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub references: Vec<Reference>,
	/// Specify career projects
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub projects: Vec<Project>,
	/// Specify side projects
	#[cfg(feature = "side-projects")]
	#[serde(rename = "sideProjects")]
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub side_projects: Vec<Project>,
	/// The schema version and any other tooling configuration lives here
	#[serde(skip_serializing_if = "Option::is_none")]
	pub meta: Option<Meta>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Basics {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// e.g. `Web Developer`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<String>,
	/// URL (as per RFC 3986) to a image in JPEG or PNG format
	#[serde(skip_serializing_if = "Option::is_none")]
	pub image: Option<String>,
	/// e.g. `thomas@gmail.com`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub email: Option<String>,
	/// Phone numbers are stored as strings so use any format you like, e.g. 712-117-2923
	#[serde(skip_serializing_if = "Option::is_none")]
	pub phone: Option<String>,
	/// URL (as per RFC 3986) to your website.
	///
	/// e.g. `https://my-blog.space`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
	/// Write a short 2-3 sentence biography about yourself
	#[serde(skip_serializing_if = "Option::is_none")]
	pub summary: Option<String>,
	pub location: Location,
	/// Specify any number of social networks that you participate in
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub profiles: Vec<Profile>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Location {
	/// To add multiple address lines, use `\n`.
	///
	/// e.g. `1234 Glücklichkeit Straße\nHinterhaus 5. Etage li.`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub address: Option<String>,
	#[serde(rename = "postalCode")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub postal_code: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub city: Option<String>,
	/// code as per ISO-3166-1 ALPHA-2.
	///
	/// e.g. `US`, `AU`, `IN`
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "countryCode")]
	pub country_code: Option<String>,
	/// The general region where you live. Can be a US state, or a province, for instance.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub region: Option<String>,
}

/// Specify any number of social networks that you participate in
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Profile {
	/// e.g. `Facebook`
	pub network: Option<String>,
	/// e.g. `neutralthoughts`
	pub username: Option<String>,
	/// e.g. `http://twitter.example.com/neutralthoughts`
	pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Work {
	/// e.g. `Facebook`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// e.g. `Menlo Park, CA`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub location: Option<String>,
	/// e.g. `Social Media Company`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/// Specify multiple positions each with a data range.
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub positions: Vec<Position>,
	/// e.g. `Software Engineer`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub position: Option<String>,
	/// e.g. `http://facebook.example.com`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
	#[serde(rename = "startDate")]
	#[cfg_attr(
		feature = "validate",
		validate(pattern = r"^([1-2][0-9]{3}(-[0-1][0-9](-[0-3][0-9])?)?)$")
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub start_date: Option<String>,
	#[serde(rename = "endDate")]
	#[cfg_attr(
		feature = "validate",
		validate(pattern = r"^([1-2][0-9]{3}(-[0-1][0-9](-[0-3][0-9])?)?)$")
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub end_date: Option<String>,
	/// Specify multiple accomplishments
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub highlights: Vec<Highlight>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Position {
	/// e.g. `Cat Herder`
	pub title: String,
	#[serde(rename = "startDate")]
	#[cfg_attr(
		feature = "validate",
		validate(pattern = r"^([1-2][0-9]{3}(-[0-1][0-9](-[0-3][0-9])?)?)$")
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub start_date: Option<String>,
	#[serde(rename = "endDate")]
	#[cfg_attr(
		feature = "validate",
		validate(pattern = r"^([1-2][0-9]{3}(-[0-1][0-9](-[0-3][0-9])?)?)$")
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub end_date: Option<String>,
}

/// e.g. `Increased profits by 20% from 2011-2012 through viral advertising`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
pub struct Highlight(pub String);

impl fmt::Display for Highlight {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Volunteer {
	/// e.g. `Facebook`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub organization: Option<String>,
	/// e.g. `Software Engineer`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub position: Option<String>,
	/// e.g. `http://facebook.example.com`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
	#[serde(rename = "startDate")]
	#[cfg_attr(
		feature = "validate",
		validate(pattern = r"^([1-2][0-9]{3}(-[0-1][0-9](-[0-3][0-9])?)?)$")
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub start_date: Option<String>,
	#[serde(rename = "endDate")]
	#[cfg_attr(
		feature = "validate",
		validate(pattern = r"^([1-2][0-9]{3}(-[0-1][0-9](-[0-3][0-9])?)?)$")
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub end_date: Option<String>,
	/// Give an overview of your responsibilities at the company
	#[serde(skip_serializing_if = "Option::is_none")]
	pub summary: Option<String>,
	/// Specify multiple accomplishments
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub highlights: Vec<Highlight>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Education {
	/// e.g. `Massachusetts Institute of Technology`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub institution: Option<String>,
	/// e.g. `http://facebook.example.com`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
	/// List of degrees or certificates awarded by this institution.
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub degrees: Vec<String>,
	/// e.g. `Arts`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub area: Option<String>,
	/// e.g. `Bachelor`
	#[serde(rename = "studyType")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub study_type: Option<String>,
	#[serde(rename = "startDate")]
	#[cfg_attr(
		feature = "validate",
		validate(pattern = r"^([1-2][0-9]{3}(-[0-1][0-9](-[0-3][0-9])?)?)$")
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub start_date: Option<String>,
	#[serde(rename = "endDate")]
	#[cfg_attr(
		feature = "validate",
		validate(pattern = r"^([1-2][0-9]{3}(-[0-1][0-9](-[0-3][0-9])?)?)$")
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub end_date: Option<String>,
	/// Grade point average.
	///
	/// e.g. `3.67/4.0`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub score: Option<String>,
	/// List notable courses/subjects
	#[serde(default)]
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub courses: Vec<Course>,
}

/// e.g. `H1302 - Introduction to American history`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
pub struct Course(pub String);

impl fmt::Display for Course {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}

/// Specify any awards you have received throughout your professional caree
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Award {
	/// e.g. `One of the 100 greatest minds of the century`
	pub title: Option<String>,
	#[cfg_attr(
		feature = "validate",
		validate(pattern = r"^([1-2][0-9]{3}(-[0-1][0-9](-[0-3][0-9])?)?)$")
	)]
	pub date: Option<String>,
	/// e.g. `Time Magazine`
	pub awarder: Option<String>,
	/// e.g. `Received for my work with Quantum Physics`
	pub summary: Option<String>,
}

/// Specify any certificates you have received throughout your professional career
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Certificate {
	/// e.g. `Certified Kubernetes Administrator`
	pub name: Option<String>,
	/// e.g. `1989-06-12`
	#[cfg_attr(
		feature = "validate",
		validate(pattern = r"^([1-2][0-9]{3}(-[0-1][0-9](-[0-3][0-9])?)?)$")
	)]
	pub date: Option<String>,
	/// e.g. `http://example.com`
	pub url: Option<String>,
	/// e.g. `CNCF`
	pub issuer: Option<String>,
}

/// Specify your publications through your career
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Publication {
	/// e.g. `The World Wide Web`
	pub name: Option<String>,
	/// e.g. `IEEE, Computer Magazine`
	pub publisher: Option<String>,
	#[serde(rename = "releaseDate")]
	#[cfg_attr(
		feature = "validate",
		validate(pattern = r"^([1-2][0-9]{3}(-[0-1][0-9](-[0-3][0-9])?)?)$")
	)]
	pub release_date: Option<String>,
	/// e.g. `http://www.computer.org.example.com/csdl/mags/co/1996/10/rx069-abs.html`
	pub url: Option<String>,
	/// Short summary of publication.
	///
	/// e.g. `Discussion of the World Wide Web, HTTP, HTML`
	pub summary: Option<String>,
}

/// List out your professional skill-set
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Skill {
	/// e.g. `Web Development`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// e.g. `Master`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub level: Option<String>,
	/// List some keywords pertaining to this skill
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub keywords: Vec<Keyword>,
}

/// e.g. `HTML`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
pub struct Keyword(pub String);

impl fmt::Display for Keyword {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}

/// List any other languages you speak
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Language {
	/// e.g. `English`, `Spanish`
	pub language: Option<String>,
	/// e.g. `Fluent`, `Beginner`
	pub fluency: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Interest {
	/// e.g. `Philosophy`
	pub name: Option<String>,
	pub keywords: Vec<Keyword>,
}

/// List references you have received
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Reference {
	/// e.g. `Timothy Cook`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/// e.g. `Joe blogs was a great employee, who turned up to work at least once a week. He exceeded my expectations when it came to doing nothing.`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reference: Option<String>,
}

/// Specify career projects
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Project {
	/// e.g. `The World Wide Web`
	pub name: Option<String>,
	/// Short summary of project.
	///
	/// e.g. `Collated works of 2017`
	pub description: Option<String>,
	/// Specify multiple features
	pub highlights: Vec<Highlight>,
	/// Specity multiple duty
	pub duties: Vec<Duty>,
	/// Specity multiple profit
	pub profits: Vec<Profit>,
	/// Specity multiple feature
	pub features: Vec<Feature>,
	/// Specify special elements involved
	pub keywords: Vec<Keyword>,
	#[serde(rename = "startDate")]
	#[cfg_attr(
		feature = "validate",
		validate(pattern = r"^([1-2][0-9]{3}(-[0-1][0-9](-[0-3][0-9])?)?)$")
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub start_date: Option<String>,
	#[serde(rename = "endDate")]
	#[cfg_attr(
		feature = "validate",
		validate(pattern = r"^([1-2][0-9]{3}(-[0-1][0-9](-[0-3][0-9])?)?)$")
	)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub end_date: Option<String>,
	/// e.g. `http://www.computer.org/csdl/mags/co/1996/10/rx069-abs.html`
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
	/// Specify your role on this project or in company
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub roles: Vec<Role>,
	/// Specify the relevant company/entity affiliations.
	///
	/// e.g. 'greenpeace', 'corporationXYZ'
	#[serde(skip_serializing_if = "Option::is_none")]
	pub entity: Option<String>,
	/// e.g. 'volunteering', 'presentation', 'talk', 'application', 'conference'
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
pub struct Feature {
	pub name: String,
	pub situation: String,
	pub task: String,
	pub action: String,
	pub result: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
pub struct Duty(pub String);

impl fmt::Display for Duty {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
pub struct Profit(pub String);

impl fmt::Display for Profit {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}
/// e.g. `Team Lead`, `Speaker`, `Writer`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
pub struct Role(pub String);

impl fmt::Display for Role {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}

/// The schema version and any other tooling configuration lives here
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "validate", derive(Validate))]
#[serde(default)]
pub struct Meta {
	/// URL (as per RFC 3986) to latest version of this document
	pub canonical: Option<String>,
	/// A version field which follows [SemVer](https://semver.org/).
	///
	/// e.g. `v1.0.0`
	pub version: Option<String>,
	/// Using ISO 8601 with `YYYY-MM-DDThh:mm:ss`
	#[serde(rename = "lastModified")]
	pub last_modified: Option<String>,
}

#[cfg(feature = "validate")]
#[cfg(test)]
mod validate {
	use super::*;

	#[test]
	fn sample() -> Result<(), Box<dyn std::error::Error>> {
		const SAMPLE: &str = include_str!("../sample.resume.json");

		let resume: Resume = serde_json::from_str(SAMPLE)?;
		resume.validate()?;

		Ok(())
	}

	#[test]
	#[ignore = "Run explicitly"]
	fn env() -> Result<(), Box<dyn std::error::Error>> {
		let resume_file = std::env::var_os("RESUME_FILE").unwrap();
		let resume = std::fs::read_to_string(resume_file)?;

		let resume: Resume = serde_json::from_str(&resume)?;
		resume.validate()?;

		println!("{resume:#?}");

		Ok(())
	}
}
