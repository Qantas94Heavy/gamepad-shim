'use strict';

require.config({
  shim: {
    'lib/iscroll': {
      exports: 'IScroll'
    }
  },
  waitSeconds: 30000
});

require(['lib/iscroll', '../../source/js/main', 'lib/require/domReady!'], function (IScroll) {
  window.addEventListener('gamepadconnected', function (event) {
    console.log(event);
  });

  window.addEventListener('gamepaddisconnected', function (event) {
    console.log(event);
  });
  
  var myScroll = new IScroll('.container.marketing', {
    snap: '.featurette'
  });
  
  $('.container.marketing').swipeEvents().on({
    swipeUp: function () {
      myScroll.prev();
    },
    swipeDown: function () {
      myScroll.next();
    }
  });
});