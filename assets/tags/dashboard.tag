<dashboard>
  <nav>
    <div class="nav-wrapper grey darken-4">
      <a href="#" class="brand-logo">Yggdrasil</a>
      <ul id="nav-mobile" class="right hide-on-med-and-down">
        <li><a>Sass</a></li>
        <li><a>Components</a></li>
        <li><a>JavaScript</a></li>
      </ul>
    </div>
  </nav>
  <div class="yggdrasil-container">
    <div class="yggdrasil-side-nav z-depth-2">
      <div class="collection">
        <a href="#!" class="collection-item nav-item grey-text text-darken-4"><i class="material-icons">dashboard</i>Dashboard</a>
        <a href="#!" class="collection-item nav-item grey-text text-darken-4"><i class="material-icons">gamepad</i> Servers</a>
        <a href="#!" class="collection-item nav-item grey-text text-darken-4"><i class="material-icons">memory</i> Infrastructure</a>
      </div>
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
