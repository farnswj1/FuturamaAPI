document.addEventListener('DOMContentLoaded', () => {
  const navbarBurgers = document.querySelectorAll('.navbar-burger');
  const active = 'is-active';

  // Add a click event on each of them
  navbarBurgers.forEach(el => {
    el.addEventListener('click', () => {
      const target = document.getElementById(el.dataset.target);
      el.classList.toggle(active);
      target.classList.toggle(active);
    });
  });
});
