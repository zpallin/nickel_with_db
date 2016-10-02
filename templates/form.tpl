{{>header}}
<body>
<h1>{{ title }}</h1>
<form id="{{ title }}" method="post">
{{#fields}}
{{&.}}
{{/fields}}
</form>
</body>
{{>footer}}
