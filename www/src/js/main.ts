import '../styles/style.css';

import initEngine, { Universe, PlayerActions } from 'engine-on-rust';

const engine = await initEngine();

const universe = new Universe();

const player = universe.create_player();

const get_russian_action_name = function (action: PlayerActions) {
    return {
        0: 'Пойти в начальную комнату',
        1: 'Пойти в другую комнату',
        2: 'Взять предмет из сундука',
        3: 'Положить предмет в сундук',
        4: 'Похвастаться предметом',
        5: 'Использовать'
    }[action];
}

function show_avalible_actions() {
    const buttons_place = document.getElementById('action-buttons')!; // '!' means - It's not null
    buttons_place.innerHTML = ""; //Clear buttons

    const avalible_actions: PlayerActions[] = universe.available_actions(player); // Generator can't get type of array arg. I helped him
    for (const action of avalible_actions) {
        // Create button for action
        let button = document.createElement("input");
        button.setAttribute('type', 'button');
        const readable_action_name = get_russian_action_name(action);
        let desc = universe.get_description()!;
        let spec_desc = document.getElementById('special-desc')!; 
        spec_desc.innerText = desc.activate;
        let expl = desc.explain;
        button.value = readable_action_name!;
        button.addEventListener('click', () => {
            let show_last_action = document.getElementById('last-action')!; 
            universe.use_action(player, action);
            show_last_action.innerText = `"${expl}" Вы выбрали "${readable_action_name}". Что теперь?`;
            show_avalible_actions();
        });
        buttons_place.appendChild(button);
    }
}

show_avalible_actions();





