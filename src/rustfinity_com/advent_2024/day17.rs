const CHRISTMAS_EMOJIS: [char; 4] = ['🎅', '🤶', '🎄', '🎁'];

pub trait Anonymize {
  fn anonymize_email(&self) -> String;
}

impl Anonymize for String {
  fn anonymize_email(&self) -> String {
    let parts = self.split_once("@");
    if parts.is_some() {
      let parts = parts.unwrap();
      let name = parts
        .0
        .chars()
        .map(|x| CHRISTMAS_EMOJIS[x as usize % CHRISTMAS_EMOJIS.len()])
        .collect::<String>();
      if parts.1.is_empty() {
        return name;
      }
      return format!("{}@{}", name, parts.1);
    }
    self
      .chars()
      .map(|x| CHRISTMAS_EMOJIS[x as usize % CHRISTMAS_EMOJIS.len()])
      .collect()
  }
}

#[test]
pub fn test_anonymize() {
  let emails = vec![
    "stuff".to_string(),
    "@northpole.com".to_string(),
    "rudolph.therapysessions@".to_string(),
    "rudolph.therapysessions@northpole.com".to_string(),
    "elfhr.complaint@northpole.urgent".to_string(),
    "santas.rage.management@christmaschaos.noel".to_string(),
    "overtimepay.never@elfexploitation.workshop".to_string(),
    "mrs.claus.divorce.lawyer@northpole.legal".to_string(),
    "reindeer.workers.comp@antler.insurance".to_string(),
    "naughty.list.revenge@santasecrets.com".to_string(),
    "workshop.ptsd.support@elves.anonymous".to_string(),
    "performance.anxiety@santa.breakdown".to_string(),
    "existential.crisis@northpole.void".to_string(),
  ];

  for email in emails {
    let anonymized_email = email.anonymize_email(); // This is the API that Santa wants!
    println!("Original: {} -> Anonymized: {}", email, anonymized_email);
  }
}
