
# Documents Guideline

This page provides the brief guideline for overall documentations.

---

- [Topic-First, Language-Later Rule](#Topic-First,%20Language-Later%20Rule)
- [English Heading & Subheading Rule](#English%20Heading%20&%20Subheading%20Rule)
- [File Naming Conventions](#File%20Naming%20Conventions)
- [Following Certain System](#Following%20Certain%20System)

## Topic-First, Language-Later Rule

When it comes to dealing with multilingual documents, we may have two following options:

- Language-First, Topic-Later Rule
	- Example 1: `domain.com/en/topic`
	- Example 2: `en.domain.com/topic/subtopic`
- Topic-First, Language-Later Rule
	- Example 1: `domain.com/topic/en`
	- Example 2: `domain.com/topic/subtopic/en`

Many documents use 'Language-First, Topic-Later' rule.

But I'm not gonna use that rule. There are several reasons for this.

- English as Primary Language
	- In most cases, I'll just stop after creating minimal English documents.
	- Thus there's no much meaning for having each language folders.
	- Even if I do, there will be only few documents.
	- So, to stick to English, I won't make separated language folders.
- Preventing Structure Differences
	- If each languages have their own folders, the folder structure can vary.
	- But, if I use 'Topic-First, Language-Later' rule, I can maintain the consitent structure among various languages simultaneously.

Also there are some additional benefits for doing this.

- Consistent Hierarchical Structure
	- If I make topic documents like `topic.md` or `topic.html`, it is quite bothering to maintain the hierarchical structure.
	- Structure Example 1 (`topic.html`)
		- `topic.html`
		- `topic/subtopic.html`
	- Structure Example 2 (`topic/index.html`)
		- `topic/index.html`
		- `topic/subtopic.html`
	- If I use `topic.html` approach, when it comes to renaming, I need to change both `topic.html` and `topic/` at the same time. If I forget it, or make mistakes, the hierarchical structure will be broken.
	- Also, it is very ambiguous if `domain.com/topic` is actually `topic.html` or `topic/index.html`. Even if I decide to stick to `topic/index.html` approach, having a folder only for a single document file doesn't look so ideal to me.
	- However, things can change if I use `topic/language.html` approach.
		- `topic/language.html`
			- `topic/subtopic/language.html`
	- As you can see, the structure is consistent.
	- Also, since the topic is already a folder, it is very easy to add any subtopics.
- Having 'README' file for overall maintenance
	- Before making any document for each topics, I can first start with creating 'README' files.
	- Each 'README' files will have specific documents guideline for covering each topics.
	- For example, maybe I can specify what is the default language of the certain topic, then I can say other languages should stay just as translations of that language.
	- Or maybe, I can set overall rules, to prevent each documents don't go far away in their own ways.

Anyway, these are why I decided to stick to 'Topic-First, Language-Later' rule.

## English Heading & Subheading Rule

The 'diversity of terms' is one of the biggest obstacles for learning something.

Experts might understand each terms, but for beginners, it's not easy.

Thus, I decided to stick to 'English Heading & Subheading' no matter what the language is.

The 'contents' of document, or certain sections can use their own terms. But, at least 'headings' and 'subheadings' should use English, for consistent maintenance.

This is also necessary for matching the order of documents in filesystem.

For example, this might happen:

- Actual Filesystem:
	- file1, file2
- English:
	- TitleOrderA(file1), TitleOrderB(file2)
- Localization
	- TitleOrderA(file2), TitleOrderB(file1)

But if we decide to stick to English headings, we can do like this:

- English:
	- EnglishTitleA(file1)
		- (English explanations)
	- EnglishTitleB(file2)
		- (English explanations)
- Localization:
	- EnglishTitleA(file1)
		- (optional heading translations)
		- (Localized explanations)
	- EnglishTitleB(file2)
		- (optional heading translations)
		- (Localized explanations)

As you can see you can also attach heading translations, if necessary.

## File Naming Conventions

Basically, we stick to 'snake_case'.

Reasons:

- Programming Habits
	- In most cases, it's not possible to use 'kebab-case' as variable names. They need to be 'snake_case', or 'PascalCase'.
	- Filesystem is possible to use 'kebab-case', but if we have to decide one, it would be better to stick to pre-existing habits.
- Unicode Symbol Order (`-`, `.`, `_`)
	- When we apply Unicode Sort for filesystem, hyphen(`-`) comes earlier than dot(`.`). This is problematique, because the order will be like this:
		- file-name.ext
		- file.ext
	- However, if we use snake_case, we can sort like this:
		- file.ext
		- file_name.ext
	- Additionally, space(` `) also comes earlier than dot. So this is not ideal too.
		- file name.ext
		- file.ext
- Upper/Lower-case
	- In most filesystems, it's not possible to use same characters with only case differences.
		- Impossible: filename.ext, FileName.ext
	- And, if we do Unicode Sorting, Upper-cases come earlier than all lower-cases.
	- Consider this:
		- CPU.md
		- Computer.md
	- Even if 'o' comes earlier than 'p', since 'P' is upper-case here, 'CPU.md' came earlier than 'Computer.md'. And this looks very problematique, since each documents have various cases. So we decide to stick to 'snake_case', with all lower cases.

However, there can be exceptions.

- Keywords, Identifiers in Programming
	- There could be things like followings:
		- PascalCase
			- StructName1
			- StructName2
		- kebab-case
			- web-element-1
			- web-element-2
	- In this case, wouldn't it be meaningless to force-apply snake cases like 'struct_name_1' or 'struct_name_2'?
	- So, if certain folder can handle topics with fixed names, or official names, it is allowed to use different naming conventions.
- Force-Applying Orders with Numbers
	- If some topics have ideal orders, it is allowed to do like this:
		- 00 Introduction
		- 01 Getting Started
	- In this case, since number will control all orders, it is free to use any naming conventions.
		- 00_Introduction
		- 01_Getting-Started
	- But, there are also rules even for this:
		- Use same digits
			- 0, 1
			- 00, 01
			- 000, 001
		- Do not use same numbers
			- Impossible: 00_Introduction, 00_Getting-Started

Also, for the sake of ordering, we have more rules than just snake_case.

- Stick to Original Words
	- Not Ideal Order:
		- academic_principle.md
		- academy.md
	- Since 'i' comes earlier than 'y', 'academic_principle.md' came earlier than 'academy.md'. And this doesn't look so ideal.
	- If we decide to stick to original words, what we get is following:
		- academy.md (Academy)
		- academy_principle.md (Academic Principle)
	- The filename does not match the title. Well in fact that is not so ideal.
	- However, since 'Academic Principle' comes later than 'Academy', this sorting looks quite reasonable.
- Change the Order of Words
	- Let's think about this:
		- Applied Science
		- Formal Science
		- Natural Science
		- Social Science
	- For last 3, I think maybe we can use followings:
		- form_science
		- nature_science
		- society_science
	- But what for 'Applied Science'?
		- 'applied_science'?
		- or specially, 'applied-science'?
		- 'apply_science'?
		- 'application_science'?
	- I said we stick to snake case 'basically', but I think there won't be any document with just name 'applied' only. So maybe we can use 'applied-science'.
	- However, it looks still problematic.
		- It allows usage of 'hyphen' anyway. This might cause confusion about what should be underscore and what should be hyphen.
		- So many documents can start with 'applied'. Thus so many unrelevant topics will be together, with the word 'applied'.
	- Starting document names with word 'applied' looks just weird to me.
	- I mean, we're not gonna make documents with name 'applied', right?
	- The situation for other words are not so different.
		- Document with just name 'apply'?
		- Document with just name 'application'?
			- (With just definition of dictionary, I mean.)
			- (Maybe we can make 'application' document for talking about 'Application Program/Software', but it's not what 'Applied Science' came from.)
	- I think 'academy_principle' doesn't look so weird, since the concepts belong to 'academy'. But, does 'Applied Science' belong to 'Application'? I don't think so. Rather than that, it's just applied-'science'.
	- So, in this case, I think just changing of word order will be fine.
		- science_applied
		- science_formal
		- science_natural
		- science_social
	- In this case, since I didn't change the word too much(like society_science or nature_science), it doesn't look so weird. Readers will understand each things mean like Applied Science or Formal Science.
	- Or maybe we can try this:
		- form
		- form_science
		- nature
		- nature_science
		- science
		- science_application
		- society
		- society_science
	- This is just up to maintainer's perspective.
	- No matter what the way is, it can be regarded as that it's following the naming rule anyway.

## Following Certain System

If some topics have certain sorting or identifying system, we use that.

- Countries (ISO Country Code)
	- country_us (United States)
	- country_gb (Great Britain, United Kingdom)
- Languages (ISO Language Code)
	- language_en (English)
	- language_es (Spanish)
- Computer File Extensions
	- Option 1: Making Divisions
		- (Benefits: Possible to Categorize)
			- lang_c (C)
			- lang_cc (C++)
			- lang_rs (Rust)
		- (Problems: Ambiguity/Decisions)
			- HTML: lang_html? view_html? web_html?
			- CSS: lang_css? view_css? web_css?
			- JSON: data_json? file_json?
			- MP3: data_mp3? file_mp3?
	- Option 2: No Strict Divisions
		- (Benefits: No Need to Decide Category or Name)
		- (Demerits: Too Mixed Up, Too Broad Category)
		- computer_c (C)
		- computer_cc (C++)
		- computer_css (CSS)
		- computer_html (HTML)
		- computer_js (JavaScript)
		- computer_mp3 (MP3)
		- computer_rs (Rust)
		- computer_ts (TypeScript)

As you can see, the overall documents structure is just entirely up to maintainers.

This page is just telling about what you can, and what you shouldn't.

---
