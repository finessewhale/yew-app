use yew::prelude::*;
use crate::layouts::HomeLayout;

#[function_component]
pub fn HomePage() -> Html {
  html!(
    <HomeLayout>
      <div class="w-full">
        <model-viewer class="h-72 lg:h-96 w-full" auto-rotate="" camera-orbit="9.698deg 72.69deg 14m" autoplay=true touch-action="pan-y" alt="Museum" src="assets/models/gameboy2.glb" shadow-intensity="4" camera-controls="" environment-image="neutral" interaction-prompt="none"></model-viewer>
      </div>

      <article class="px-4 text-white mb-6">
        <div class="-mt-8 relative bg-white bg-opacity-10 text-center rounded-lg w-full p-4 mb-6 font-text backdrop-blur-lg">
          {"Welcome to my vault. I'm an indie app developer based in Odesa!"}
        </div>

        <div class="flex flex-col text-left text-white mb-6">
          <h2 class="font-bold text-3xl">
            {"Maxim Fineskin"}
          </h2>
          <p class="font-text">
            {"Digital Craftsman ( Engineer / Developer / Designer )"}
          </p>
        </div>

        <div>
          <h3 class="my-4 decoration-neutral-700 decoration-4 underline underline-offset-8 text-xl font-bold">
            {"Work"}
          </h3>
          <p class="indent-4 hyphens-auto text-justify font-text">
            {
              "Max is a freelance and a full-stack developer based in Odesa with a passion for building digital services/stuff he wants.
              He has a knack for all things launching products, from planning and designing all the way to solving real-life problems with code.
              When not online, he loves climbing mountains and maintain his servers. Currently, he is living off of remote *nix systems maintaining/administration"
            }
          </p>
        </div>

        <div>
          <h3 class="my-4 decoration-neutral-700 decoration-4 underline underline-offset-8 text-xl font-bold">
            {"Bio"}
          </h3>
          <div class="pl-14 -indent-14">
            <span class="mr-4 font-bold">
              {"2002"}
            </span>
            {"Born in Odesa, Ukraine."}
          </div>
          <div class="pl-14 -indent-14">
            <span class="mr-4 font-bold">
              {"2021"}
            </span>
            {
              "Graduated with a degree in Junior Specialist in Computer Graphics and Web Design"
            }
          </div>
          <div class="pl-14 -indent-14">
            <span class="mr-4 font-bold">{"2019-Now"}</span>
            {"Working as a freelancer"}
          </div>
        </div>

        <div>
          <h3 class="my-4 decoration-neutral-700 decoration-4 underline underline-offset-8 text-xl font-bold">
            {"I <3"}
          </h3>
          <p class="indent-4 hyphens-auto text-justify font-text">
            {
              "Art, Music, Microcontrollers, Restoration of old electronics, Photography, Leica, Machine Learning"
            }
          </p>
        </div>
      </article>
    </HomeLayout>
  )
}
