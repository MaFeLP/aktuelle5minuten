<script lang="ts">
  import homeIcon from "bootstrap-icons/icons/house-fill.svg";
  import newsLogo from "/news.svg";

  export let title: string;
</script>

<nav class="navbar navbar-expand-lg navbar-light bg-light">
  <div class="container-fluid">
    <a class="navbar-brand" href="/">
      <img id="home-icon" src={homeIcon} alt="Home Icon" />
    </a>
    <h1 class="nav-item">{title}</h1>
    <img id="news-icon" src={newsLogo} alt="News Icon" />
  </div>
</nav>

<style lang="sass">
  .navbar-brand
    display: flex
    align-content: center
    place-items: center

  #home-icon, #news-icon
    height: 1.5rem
</style>
