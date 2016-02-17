<dashboard>
  <nav>
    <div class="nav-wrapper grey darken-3">
      <a href="#" class="brand-logo">Yggdrasil</a>
      <ul id="nav-mobile" class="right hide-on-med-and-down">
        <li><a>Sass</a></li>
        <li><a>Components</a></li>
        <li><a>JavaScript</a></li>
      </ul>
    </div>
  </nav>
  <div class="row">
    <div class="col s12 m4 l3">

    </div>

    <div class="col s12 m8 l9">

    </div>

  </div>
  <script>
    import api from "../js/api.js";

    api.get_session().then((session) => {

    }, (err) => {
      riot.route("login");
    });
  </script>
</dashboard>
