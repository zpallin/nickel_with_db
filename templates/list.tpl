{{>header}}
<!-- List View -->
<body>
<h1>{{ title }}</h1>
<ul>
{{#values}}
<li>{{.}}</li>
{{/values}}
</ul>
</body>
{{>footer}}
