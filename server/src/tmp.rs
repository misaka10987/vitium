type REG<T> = Lazy<Mutex<RegTable<T>>>;
macro_rules! reg {
    () => {
        Lazy::new(|| Mutex::new(RegTable::new()))
    };
}
static REG_ITEM: REG<Item> = reg!();
static REG_SKILL: REG<Skill> = reg!();
static REG_PROF: REG<Prof> = reg!();
static REG_SCENE: REG<Scene> = reg!();
static REG_VEHICLE: REG<Vehicle> = reg!();