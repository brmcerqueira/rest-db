import "../../rest-db"

type Employee = {
    entityId: Number,
    firstname: string,
    employeeTerritory?: string[]
    territory?: Territory[]
}

type EmployeeTerritory = {
    entityId: number,
    employeeId: number,
    territoryCode: string
}

type Territory  = {
    entityId: number,
    regionId: number,
    territoryCode: string,
    territorydescription: string,
    region?: Region
}

type Region = {
    entityId: number,
    regiondescription: string
}

export function queryEmployee(args: { text: string }) {
    $collection("Employee");
    $filter<Employee>(employee => employee.firstname.includes(args.text));
    $lookup<Employee, string>("EmployeeTerritory", (employee) => employee.employeeTerritory = $all(), employee => {
        $filter<EmployeeTerritory>(et => et.employeeId == employee.entityId);
        $project<EmployeeTerritory>(et => et.territoryCode);
    });
    $lookup<Employee, Territory>("Territory", (employee) => employee.territory = $all(), employee => {
        $filter<Territory>(territory => employee.employeeTerritory.indexOf(territory.territoryCode) > -1);
        $lookup<Territory, Region>("Region", (territory) => territory.region = $first(), territory => {
            $filter<Region>(region => territory.regionId == region.entityId);
        });
    });
    $project<Employee>(employee => {
        const { employeeTerritory, ...all } = employee;
        return all;
    });
    $group<Employee, any, any>(employee => employee.territory[0].regionId, key => {
        return { key, amount: $sum<Employee>(item => item.entityId), data: $all() };
    })
}