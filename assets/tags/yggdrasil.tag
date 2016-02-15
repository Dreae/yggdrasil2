<yggdrasil>
  <div id="view"></div>
  <script>
    import './login-page.tag';
    import './dashboard.tag';

    let self = this;
    self.currentView = null;

    let mount = (tag, options) => {
      if(self.currentView) {
        self.currentView.unmount(true);
      }

      self.currentView = riot.mount('div#view', tag, options)[0];
    }

    riot.route((collection, id, action) => {
      if(collection == "login") {
        mount('login-page');
        self.currentView.trigger('route', collection, id, action);
      } else if(collection == "dashboard") {
        mount('dashboard');
        self.currentView.trigger('route', collection, id, action);
      } else {
        riot.route('dashboard');
      }
    })
  </script>
</yggdrasil>
