<!DOCTYPE html>
<html lang="en">
<head>
	<meta http-equiv="content-type" content="text/html; charset=UTF-8">
	<meta charset="utf-8">
	<title>Home | HTML5 Gamepad Shim</title>
	<meta name="generator" content="Bootply" />
	<meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1">
	<link href="css/lib/bootstrap.min.css" rel="stylesheet">
	<!--[if lt IE 9]>
		<script src="js/lib/html5.min.js"></script>
	<![endif]-->
	<link href="css/styles.css" rel="stylesheet">
</head>
<body>
	<div class="navbar-wrapper">
		<div class="container">
			<div class="navbar navbar-inverse navbar-static-top">
				
				<div class="navbar-header">
          <a class="navbar-toggle" data-toggle="collapse" data-target=".navbar-collapse">
            <span class="icon-bar"></span>
            <span class="icon-bar"></span>
            <span class="icon-bar"></span>
          </a>
					<a class="navbar-brand" href="#">HTML5 Gamepad Shim</a>
					</div>
					<div class="navbar-collapse collapse">
						<ul class="nav navbar-nav">
							<li class="active"><a href="#">Home</a></li>
							<li><a href="about.html" target="ext">About</a></li>
							<li><a href="download.html">Download</a></li>
							<li class="dropdown">
								<a href="#" class="dropdown-toggle" data-toggle="dropdown">Tutorial <b class="caret"></b></a>
								<ul class="dropdown-menu">
									<li><a href="tutorial/js.html#">Flash (AS3) side</a></li>
                  <li class="divider"></li>
									<li><a href="tutorial/js.html#">JavaScript side</a></li>
								</ul>
							</li>
						</ul>
					</div>

			</div>
		</div><!-- /container -->
	</div><!-- /navbar wrapper -->


	<!-- Carousel
	================================================== -->
	<div id="myCarousel" class="carousel slide">
		<!-- Indicators -->
		<ol class="carousel-indicators">
			<li data-target="#myCarousel" data-slide-to="0" class="active"></li>
			<li data-target="#myCarousel" data-slide-to="1"></li>
			<li data-target="#myCarousel" data-slide-to="2"></li>
		</ol>
		<div class="carousel-inner">
			<div class="item active">
				<img src="http://lorempizza.com/1500/500" class="img-responsive">
				<div class="container">
					<div class="carousel-caption">
						<h1>Bootstrap 3 Carousel Layout</h1>
						<p><a class="btn btn-lg btn-primary" href="http://getbootstrap.com">Learn More</a></p>
          </div>
				</div>
			</div>
			<div class="item">
				<img src="http://lorempizza.com/1500/500" class="img-responsive">
				<div class="container">
					<div class="carousel-caption">
						<h1>Changes to the Grid</h1>
						<p>Bootstrap 3 still features a 12-column grid, but many of the CSS class names have completely changed.</p>
						<p><a class="btn btn-large btn-primary" href="#">Learn more</a></p>
					</div>
				</div>
			</div>
			<div class="item">
				<img src="http://placehold.it/1500X500" class="img-responsive">
				<div class="container">
					<div class="carousel-caption">
						<h1>Percentage-based sizing</h1>
						<p>With "mobile-first" there is now only one percentage-based grid.</p>
						<p><a class="btn btn-large btn-primary" href="#">Browse gallery</a></p>
					</div>
				</div>
			</div>
		</div>
		<!-- Controls -->
		<a class="left carousel-control" href="#myCarousel" data-slide="prev">
			<span class="icon-prev"></span>
		</a>
		<a class="right carousel-control" href="#myCarousel" data-slide="next">
			<span class="icon-next"></span>
		</a>  
	</div>
	<!-- /.carousel -->


	<!-- Marketing messaging and featurettes
	================================================== -->
	<!-- Wrap the rest of the page in another container to center all the content. -->

	<div class="container marketing">

		<!-- Three columns of text below the carousel -->
		<div class="row">
			<div class="col-md-4 text-center">
				<img class="img-circle" src="http://placehold.it/140x140">
				<h2>Mobile-first</h2>
				<p>Tablets, phones, laptops. The new 3 promises to be mobile friendly from the start.</p>
				<p><a class="btn btn-default" href="#">View details »</a></p>
			</div>
			<div class="col-md-4 text-center">
				<img class="img-circle" src="http://placehold.it/140x140">
				<h2>One Fluid Grid</h2>
				<p>There is now just one percentage-based grid for Bootstrap 3. Customize for fixed widths.</p>
				<p><a class="btn btn-default" href="#">View details »</a></p>
			</div>
			<div class="col-md-4 text-center">
				<img class="img-circle" src="http://placehold.it/140x140">
				<h2>LESS is More</h2>
				<p>Improved support for mixins make the new Bootstrap 3 easier to customize.</p>
				<p><a class="btn btn-default" href="#">View details »</a></p>
			</div>
		</div><!-- /.row -->


		<!-- START THE FEATURETTES -->

		<hr class="featurette-divider">

		<div class="featurette">
			<img class="featurette-image img-circle pull-right" src="img/iesucks.png">
			<h2 class="featurette-heading">More browsers. <span class="text-muted">No more worrying about IE.</span></h2>
			<p class="lead">Ever had concerns about compatibility for different APIs on different browsers? Chuck your fears away with the HTML5 Gamepad Shim.</p>
		</div>

		<hr class="featurette-divider">

		<!-- /END THE FEATURETTES -->


		<!-- FOOTER -->
		<footer>
			<p class="pull-right"><a href="#">Back to top</a></p>
		</footer>

	</div><!-- /.container -->
	
	<!-- script references -->
	<script src="js/lib/jquery-2.1.1.min.js"></script>
  <script src="js/lib/jquery.swipe-events.js"></script>
	<script src="js/lib/bootstrap.min.js"></script>
	<script src="js/lib/require/require.js" data-main="js/main.js"></script>
</body>
</html>