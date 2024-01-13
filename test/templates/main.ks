<!--
Kismesis has templates and content files,
This is a template. It contains stuff that will be on various pages
The content files are the individual pages, like for example, a page in a webcomic
or an article in a blog.
If I change something here, it'll change in all the pages in the output
-->

const examplevar = "My variable" <!-- Variables are useful when you need to reuse some text a lot -->
const otherVariable = examplevar  <!-- Variables can reference each other. This variable is *also* equal to "My variable" -->
mut title  <!-- Lambdas are like variables but their value can be changed by the individual content files -->
<!-- If you don't set a lambda's value (like I did here), then you'll have to set its value in the content file -->

<!doctype html>

<!-- Instead of using HTML, kismesis uses a custom format that then becomes HTML when you run the build command -->
<html lang="en-US":
	<head:
		<link href="/style.css" rel="stylesheet" type="text/css" media="all">
		<meta name="viewport" content="width=device-width, initial-scale=1">
		<meta charset="utf-8">
		<!-- This pattern is called a "ternary IF"
		     {{CONDITION and IF_SUCCESS} or IF_FAILS}
		     Here, I'm saying that, if a page sets the title, then here it should say "Ampersandia - " followed by that title
			 But if I didn't do that, then it should use only the word "Ampersandia"
			 If I don't do this, then the program will warn me that I didn't set the value
		-->
		<if title:
			<title | @title >
		>
		<if {not title}:
			<title | Ampersandia >
		>
	>
	<body:
		<header:
			<span id="logo" | &>
        >
		<!-- You can use #this for headers. This is only for headers, and not for large text. If you want large text, use CSS with regular text -->
		<!-- This is to make the page more accessible to screen readers -->
		<!-- This is similar to #doing this in markdown -->
				
        <content!>  <!-- This is the content mark. The page will put all the content files in the spot where you put this mark -->
    >
>
